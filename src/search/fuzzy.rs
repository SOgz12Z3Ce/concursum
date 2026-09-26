use crate::{
    error::Error,
    search::{
        Fields, SearchEngine, SearchResult,
        snippet::{self, FuzzySnippet, GeneralSnippet},
    },
};
use tantivy::{
    TantivyDocument,
    collector::TopDocs,
    query::QueryParser,
    query_grammar::{self, UserInputAst, UserInputLeaf, UserInputLiteral},
    schema::Value,
};

pub(crate) fn search(
    search_engine: &SearchEngine,
    query: &str,
) -> Result<Vec<SearchResult>, Error> {
    let Ok(ast) = query_grammar::parse_query(query) else {
        return Err(Error::from_query(query.to_owned()));
    };
    let literals = literals(&ast);

    // Collect phrases by field.
    let mut label_phrases: Vec<&str> = Vec::new();
    let mut description_phrases: Vec<&str> = Vec::new();
    for literal in literals {
        let phrase = &literal.phrase;
        match &literal.field_name {
            Some(field_name) if field_name == "名称" => label_phrases.push(phrase),
            Some(field_name) if field_name == "描述" => description_phrases.push(phrase),
            Some(field_name) => return Err(Error::NotsupportedQuery(field_name.to_owned())),
            None => {
                label_phrases.push(phrase.as_str());
                description_phrases.push(phrase.as_str());
            }
        }
    }
    let label_phrases = label_phrases;
    let description_phrases = description_phrases;

    // Check phrase lengths.
    if !can_fuzzy(&label_phrases) || !can_fuzzy(&description_phrases) {
        return Err(Error::BadFuzzySearch);
    }

    // Create query.
    let SearchEngine {
        index,
        fields:
            Fields {
                index: index_field,
                label: label_field,
                description: description_field,
            },
    } = search_engine;
    let default_fields = vec![*label_field, *description_field];
    let mut parser = QueryParser::for_index(index, default_fields);
    parser.set_field_fuzzy(*label_field, false, 1, true);
    parser.set_field_fuzzy(*description_field, false, 1, true);
    let query = parser.parse_query(query)?;

    let searcher = index.reader()?.searcher();
    let label_tokenizer = searcher.index().tokenizer_for_field(*label_field)?;
    let description_tokenizer = searcher.index().tokenizer_for_field(*description_field)?;
    let collector = TopDocs::with_limit(100).order_by_score();
    let documents: Vec<TantivyDocument> = searcher
        .search(&query, &collector)?
        .into_iter()
        .map(|(_, doc_address)| {
            let document = searcher.doc(doc_address)?;
            Ok(document)
        })
        .collect::<Result<_, Error>>()?;
    let mut results: Vec<(SearchResult, bool)> = documents
        .iter()
        .map(|document| {
            let index = document
                .get_first(*index_field)
                .expect("every document has a index")
                .as_u64()
                .expect("index is a usize") as usize; // Since we don't have 4.3B objects.

            let label_snippets = document
                .get_all(*label_field)
                .map(|label| label.as_str().expect("label is a string"))
                .map(|label| {
                    snippet::fuzzy_snippet(label_tokenizer.clone(), &label_phrases, label)
                });
            let description_snippets = document
                .get_all(*description_field)
                .map(|description| description.as_str().expect("description is a string"))
                .map(|description| {
                    snippet::fuzzy_snippet(
                        description_tokenizer.clone(),
                        &description_phrases,
                        description,
                    )
                });
            let snippets: Vec<(Vec<FuzzySnippet>, bool)> = label_snippets
                .chain(description_snippets)
                .filter(|(snippet, _)| !snippet.is_empty())
                .collect();
            let full_match = snippets.iter().any(|(_, full_match)| *full_match);
            let snippets = snippets
                .into_iter()
                .flat_map(|(snippet, _)| snippet)
                .map(|snippet| Box::new(snippet) as Box<dyn GeneralSnippet>)
                .collect();

            (SearchResult { index, snippets }, full_match)
        })
        .collect();
    results.sort_by_key(|(_, full_match)| !full_match);
    let results = results.into_iter().map(|(snippet, _)| snippet).collect();
    Ok(results)
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

fn can_fuzzy(phrases: &Vec<&str>) -> bool {
    phrases.iter().all(|phrase| phrase.chars().count() != 1)
}
