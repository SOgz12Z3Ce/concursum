mod snippet;

use crate::{data::cs::DataView, error::Error, search::snippet::GeneralSnippet};
use tantivy::{
    Index, TantivyDocument,
    collector::TopDocs,
    query::QueryParser,
    query_grammar::{self, UserInputAst, UserInputLeaf, UserInputLiteral},
    schema::{
        FAST, Field, IndexRecordOption, STORED, Schema, TextFieldIndexing, TextOptions, Value,
    },
    snippet::{Snippet, SnippetGenerator},
    tokenizer::NgramTokenizer,
};
// use tantivy_jieba::JiebaTokenizer;

static NGRAM_TOKENIZER_NAME: &'static str = "1_2-gram";
static INDEX_FIELD_NAME: &'static str = "index";
static LABEL_FIELD_NAME: &'static str = "名称";
static DESCRIPTION_FIELD_NAME: &'static str = "描述";
static MEMORY_BUDGET: usize = 50_000_000; // TODO: Make this configurable.

#[derive(Debug)]
pub(crate) struct SearchEngine {
    pub(crate) index: Index,
    pub(crate) fields: Fields,
}

#[derive(Debug)]
pub(crate) struct Fields {
    pub(crate) index: Field,
    pub(crate) label: Field,
    pub(crate) description: Field,
}

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub(crate) index: usize,
    pub(crate) snippets: Vec<Box<dyn GeneralSnippet>>,
}

pub(crate) fn index(data_view: &DataView) -> Result<SearchEngine, Error> {
    let Fields {
        index: index_field,
        label: label_field,
        description: description_field,
    };
    let schema = {
        let text_option = TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    // .set_tokenizer(JIEBA_TOKENIZER_NAME)
                    .set_tokenizer(NGRAM_TOKENIZER_NAME)
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored();

        let mut builder = Schema::builder();
        index_field = builder.add_u64_field(INDEX_FIELD_NAME, FAST | STORED);
        label_field = builder.add_text_field(LABEL_FIELD_NAME, text_option.clone());
        description_field = builder.add_text_field(DESCRIPTION_FIELD_NAME, text_option);
        builder.build()
    };
    let index = Index::create_in_ram(schema);
    index.tokenizers().register(
        // JIEBA_TOKENIZER_NAME,
        // JiebaTokenizer::default(),
        NGRAM_TOKENIZER_NAME,
        NgramTokenizer::new(2, 2, false).unwrap(),
    );

    let mut writer = index.writer(MEMORY_BUDGET)?;
    for (index, object) in data_view.objects().iter().enumerate() {
        let summary = object.summary()?;
        let mut document = TantivyDocument::default();
        document.add_u64(index_field, index as u64);
        for label in summary.labels() {
            document.add_text(label_field, label);
        }
        for description in summary.descriptions() {
            document.add_text(description_field, description);
        }
        writer.add_document(document)?;
    }
    writer.commit()?;
    Ok(SearchEngine {
        index,
        fields: Fields {
            index: index_field,
            label: label_field,
            description: description_field,
        },
    })
}

pub(crate) fn search(
    cs_index: &SearchEngine,
    keywords: &String,
) -> Result<Vec<SearchResult>, Error> {
    let SearchEngine {
        index,
        fields:
            Fields {
                index: index_field,
                label: label_field,
                description: description_field,
            },
    } = cs_index;
    let default_fields = vec![*label_field, *description_field];
    let mut parser = QueryParser::for_index(index, default_fields.clone());
    let ast = query_grammar::parse_query(keywords)
        .map_err(|_| Error::QueryGrammar(keywords.to_owned()))?;
    let literals = literals(&ast);

    // Collect phrases by field.
    let mut label_phrases = Vec::new();
    let mut description_phrases = Vec::new();
    for literal in literals {
        let phrase = &literal.phrase;
        match &literal.field_name {
            Some(field_name) if field_name == "名称" => label_phrases.push(phrase),
            Some(field_name) if field_name == "描述" => description_phrases.push(phrase),
            Some(field_name) => return Err(Error::NotsupportedQuery(field_name.to_owned())),
            None => {
                label_phrases.push(phrase);
                description_phrases.push(phrase);
            }
        }
    }
    let label_phrases = label_phrases;
    let description_phrases = description_phrases;

    // Check one word fields.
    let label_is_fuzzy = is_fuzzy(&label_phrases);
    let description_is_fuzzy = is_fuzzy(&description_phrases);

    // Set field as fuzzy.
    if label_is_fuzzy {
        parser.set_field_fuzzy(*label_field, false, 1, true);
    }
    if description_is_fuzzy {
        parser.set_field_fuzzy(*description_field, false, 1, true);
    }
    let query = parser.parse_query(keywords)?;

    // TODO:
    // Choose a better strategy to collect docs.
    // Exact matched docs MUST be collected, while fuzzy matched docs SHOULD be
    // collected with a limit.
    let searcher = index.reader()?.searcher();
    let default_label_snippet_generator =
        SnippetGenerator::create(&searcher, &query, *label_field)?;
    let default_description_snippet_generator =
        SnippetGenerator::create(&searcher, &query, *description_field)?;
    let label_tokenizer = searcher.index().tokenizer_for_field(*label_field)?;
    let description_tokenizer = searcher.index().tokenizer_for_field(*description_field)?;
    searcher
        .search(&query, &TopDocs::with_limit(100).order_by_score())?
        .into_iter()
        .map(|(_, address)| {
            let doc: TantivyDocument = searcher.doc(address)?;
            let index = doc
                .get_first(*index_field)
                .expect("every document has a index")
                .as_u64()
                .expect("index is a usize") as usize; // Since we don't have 4.3B objects.

            let labels = doc
                .get_all(*label_field)
                .map(|label| label.as_str().expect("label is a string"));
            let label_snippets = labels.flat_map(|label| {
                if label_is_fuzzy {
                    snippet::fuzzy_snippet(label_tokenizer.clone(), &label_phrases, label)
                } else {
                    let snippet: Box<dyn GeneralSnippet> =
                        Box::new(default_label_snippet_generator.snippet(label));
                    vec![snippet]
                    // vec![Box::new(default_label_snippet_generator.snippet(label))]
                }
            });

            let descriptions = doc
                .get_all(*description_field)
                .map(|description| description.as_str().expect("description is a string"));
            let description_snippets = descriptions.flat_map(|description| {
                if description_is_fuzzy {
                    snippet::fuzzy_snippet(
                        description_tokenizer.clone(),
                        &description_phrases,
                        description,
                    )
                } else {
                    let snippet: Box<dyn GeneralSnippet> =
                        Box::new(default_description_snippet_generator.snippet(description));
                    vec![snippet]
                }
            });

            let snippets = label_snippets
                .chain(description_snippets)
                .filter(|snippet| !snippet.is_empty())
                .collect();
            Ok(SearchResult { index, snippets })
        })
        .collect()
}

fn literals(ast: &UserInputAst) -> Vec<&UserInputLiteral> {
    match ast {
        UserInputAst::Clause(items) => items.iter().flat_map(|(_, ast)| literals(ast)).collect(),
        UserInputAst::Boost(user_input_ast, _) => literals(user_input_ast),
        UserInputAst::Leaf(user_input_leaf) => match &**user_input_leaf {
            UserInputLeaf::Literal(user_input_literal) => vec![&user_input_literal],
            _ => vec![],
        },
    }
}

fn is_fuzzy(phrases: &Vec<&String>) -> bool {
    phrases.iter().all(|phrase| phrase.chars().count() != 1)
}
