use crate::data::DATA;
use std::{collections::HashMap, sync::LazyLock};
use tantivy::{
    Index, TantivyDocument,
    collector::TopDocs,
    doc,
    query::QueryParser,
    schema::{
        FAST, Field, IndexRecordOption, STORED, Schema, TextFieldIndexing, TextOptions, Value,
    },
    snippet::{Snippet, SnippetGenerator},
};
use tantivy_jieba::JiebaTokenizer;

static JIEBA_TOKENIZER_NAME: &'static str = "jieba";
static INDEX_FIELD_NAME: &'static str = "index";
static LABEL_FIELD_NAME: &'static str = "label";
static DESCRIPTION_FIELD_NAME: &'static str = "description";
static MEMORY_BUDGET: usize = 50_000_000;

static KEYWORDS_PARAM_NAME: &'static str = "keywords";

struct SearchEngine {
    pub(crate) index: Index,
    pub(crate) fields: Fields,
}

struct Fields {
    pub(crate) index: Field,
    pub(crate) label: Field,
    pub(crate) description: Field,
}

pub(crate) struct SearchResult {
    pub(crate) index: usize,
    pub(crate) snippet: Snippet,
}

static INDEX: LazyLock<SearchEngine> = LazyLock::new(|| {
    let Fields {
        index: index_field,
        label: label_field,
        description: description_field,
    };
    let schema = {
        let mut builder = Schema::builder();
        index_field = builder.add_u64_field(INDEX_FIELD_NAME, FAST | STORED);
        label_field = builder.add_text_field(
            LABEL_FIELD_NAME,
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer(JIEBA_TOKENIZER_NAME)
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
        description_field = builder.add_text_field(
            DESCRIPTION_FIELD_NAME,
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer(JIEBA_TOKENIZER_NAME)
                        .set_index_option(IndexRecordOption::WithFreqsAndPositions),
                )
                .set_stored(),
        );
        builder.build()
    };
    let index = Index::create_in_ram(schema);
    index
        .tokenizers()
        .register(JIEBA_TOKENIZER_NAME, JiebaTokenizer::default());

    let mut writer = index.writer(MEMORY_BUDGET).unwrap();
    let texts = DATA.texts();
    for index in 0..DATA.localized_objects.len() {
        let mut document = doc!(
            index_field => index as u64,
        );
        if let Some(label) = texts.labels[index] {
            document.add_text(label_field, label);
        }
        if let Some(description) = texts.descriptions[index] {
            document.add_text(description_field, description);
        }
        writer.add_document(document).unwrap();
    }
    writer.commit().unwrap();

    SearchEngine {
        index,
        fields: Fields {
            index: index_field,
            label: label_field,
            description: description_field,
        },
    }
});

pub(crate) fn search(params: HashMap<String, String>) -> Vec<SearchResult> {
    let Some(keyword) = params.get(KEYWORDS_PARAM_NAME) else {
        return Vec::new();
    };

    let SearchEngine {
        index,
        fields:
            Fields {
                index: _,
                label: label_field,
                description: description_field,
            },
    } = &*INDEX;
    let index_field = index.schema().get_field(INDEX_FIELD_NAME).unwrap();

    let default_fields = vec![*label_field, *description_field];
    let parser = QueryParser::for_index(index, default_fields);
    let query = parser.parse_query(keyword).unwrap();

    // TODO:
    // Choose a better strategy to collect docs.
    // Exact matched docs MUST be collected, while fuzzy matched docs SHOULD be
    // collected with a limit.
    let searcher = index.reader().unwrap().searcher();
    let description_snippet_generator =
        SnippetGenerator::create(&searcher, &query, *description_field).unwrap();
    let label_snippet_generator =
        SnippetGenerator::create(&searcher, &query, *label_field).unwrap();
    searcher
        .search(&query, &TopDocs::with_limit(100).order_by_score())
        .unwrap()
        .into_iter()
        .map(|(_, address)| {
            let doc: TantivyDocument = searcher.doc(address).unwrap();
            let index = doc.get_first(index_field).unwrap().as_u64().unwrap() as usize; // Since we don't have 4.3B objects
            let mut snippet = description_snippet_generator.snippet_from_doc(&doc);
            if snippet.is_empty() {
                snippet = label_snippet_generator.snippet_from_doc(&doc);
            }

            SearchResult { index, snippet }
        })
        .collect()
}
