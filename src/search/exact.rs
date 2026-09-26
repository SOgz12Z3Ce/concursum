use crate::{
    error::Error,
    search::{Fields, SearchEngine, SearchResult, snippet::GeneralSnippet},
};
use tantivy::{
    TantivyDocument, collector::TopDocs, query::QueryParser, schema::Value,
    snippet::SnippetGenerator,
};

pub(crate) fn search(
    search_engine: &SearchEngine,
    query: &str,
) -> Result<Vec<SearchResult>, Error> {
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
    let parser = QueryParser::for_index(index, default_fields);
    let query = parser.parse_query(query)?;

    let searcher = index.reader()?.searcher();
    let label_snippet_generator = SnippetGenerator::create(&searcher, &query, *label_field)?;
    let description_snippet_generator =
        SnippetGenerator::create(&searcher, &query, *description_field)?;

    let collector = TopDocs::with_limit(100).order_by_score();
    let documents: Vec<TantivyDocument> = searcher
        .search(&query, &collector)?
        .into_iter()
        .map(|(_, doc_address)| {
            let document = searcher.doc(doc_address)?;
            Ok(document)
        })
        .collect::<Result<_, Error>>()?;
    let results = documents
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
                .map(|label| label_snippet_generator.snippet(label));
            let description_snippets = document
                .get_all(*description_field)
                .map(|description| description.as_str().expect("description is a string"))
                .map(|description| description_snippet_generator.snippet(description));
            let snippets = label_snippets
                .chain(description_snippets)
                .filter(|snippet| !snippet.is_empty())
                .map(|snippet| Box::new(snippet) as Box<dyn GeneralSnippet>)
                .collect();

            SearchResult { index, snippets }
        })
        .collect();
    Ok(results)
}
