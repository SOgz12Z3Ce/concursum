use crate::{data::cs::DataView, error::Error};
use tantivy::{
    Index, TantivyDocument,
    collector::TopDocs,
    query::QueryParser,
    schema::{
        FAST, Field, IndexRecordOption, STORED, Schema, TextFieldIndexing, TextOptions, Value,
    },
    snippet::{Snippet, SnippetGenerator},
};
use tantivy_jieba::JiebaTokenizer;

static JIEBA_TOKENIZER_NAME: &'static str = "jieba";
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

pub(crate) struct SearchResult {
    pub(crate) index: usize,
    pub(crate) snippets: Vec<Snippet>,
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
                    .set_tokenizer(JIEBA_TOKENIZER_NAME)
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
    index
        .tokenizers()
        .register(JIEBA_TOKENIZER_NAME, JiebaTokenizer::default());

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
    let parser = QueryParser::for_index(index, default_fields);
    let query = parser.parse_query(keywords)?;

    // TODO:
    // Choose a better strategy to collect docs.
    // Exact matched docs MUST be collected, while fuzzy matched docs SHOULD be
    // collected with a limit.
    let searcher = index.reader()?.searcher();
    let label_snippet_generator = SnippetGenerator::create(&searcher, &query, *label_field)?;
    let description_snippet_generator =
        SnippetGenerator::create(&searcher, &query, *description_field)?;
    searcher
        .search(&query, &TopDocs::with_limit(50_000).order_by_score())?
        .into_iter()
        .map(|(_, address)| {
            let doc: TantivyDocument = searcher.doc(address)?;
            let index = doc
                .get_first(*index_field)
                .expect("every document has a index")
                .as_u64()
                .expect("index is a usize") as usize; // Since we don't have 4.3B objects.
            let label_snippets = doc.get_all(*label_field).into_iter().map(|label| {
                label_snippet_generator.snippet(label.as_str().expect("label is a string"))
            });
            let description_snippets = doc.get_all(*label_field).into_iter().map(|description| {
                description_snippet_generator
                    .snippet(description.as_str().expect("description is a string"))
            });

            let snippets = label_snippets
                .chain(description_snippets)
                .filter(|snippet| !snippet.is_empty())
                .collect();
            Ok(SearchResult { index, snippets })
        })
        .collect()
}
