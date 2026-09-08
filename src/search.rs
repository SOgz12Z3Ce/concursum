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
};
use tantivy_jieba::JiebaTokenizer;

static JIEBA_TOKENIZER_NAME: &'static str = "jieba";
static INDEX_FIELD_NAME: &'static str = "index";
static LABEL_FIELD_NAME: &'static str = "label";
static MEMORY_BUDGET: usize = 50_000_000;

static KEYWORDS_PARAM_NAME: &'static str = "keywords";

struct SearchEngine {
    pub(crate) index: Index,
    pub(crate) label_field: Field,
}

static INDEX: LazyLock<SearchEngine> = LazyLock::new(|| {
    let index_field;
    let label_field;
    let schema = {
        let mut builder = Schema::builder();
        index_field = builder.add_u64_field(INDEX_FIELD_NAME, FAST | STORED);
        label_field = builder.add_text_field(
            LABEL_FIELD_NAME,
            TextOptions::default().set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer(JIEBA_TOKENIZER_NAME)
                    .set_index_option(IndexRecordOption::Basic),
            ),
        );
        builder.build()
    };
    let index = Index::create_in_ram(schema);
    index
        .tokenizers()
        .register(JIEBA_TOKENIZER_NAME, JiebaTokenizer::default());

    let mut writer = index.writer(MEMORY_BUDGET).unwrap();
    for (index, content) in DATA.objects.contents.iter().enumerate() {
        if let Some(label) = content.get(LABEL_FIELD_NAME).and_then(|v| v.as_str()) {
            writer
                .add_document(doc!(
                    index_field => index as u64,
                    label_field => label
                ))
                .unwrap();
        }
    }
    writer.commit().unwrap();

    SearchEngine { index, label_field }
});

pub(crate) fn search(params: HashMap<String, String>) -> Vec<usize> {
    let Some(keyword) = params.get(KEYWORDS_PARAM_NAME) else {
        return Vec::new();
    };

    let SearchEngine { index, label_field } = &*INDEX;
    let index_field = index.schema().get_field(INDEX_FIELD_NAME).unwrap();

    let default_fields = vec![*label_field];
    let parser = QueryParser::for_index(index, default_fields);
    let query = parser.parse_query(keyword).unwrap();

    // TODO:
    // Choose a better strategy to collect docs.
    // Exact matched docs MUST be collected, while fuzzy matched docs SHOULD be
    // collected with a limit.
    let searcher = index.reader().unwrap().searcher();
    searcher
        .search(&query, &TopDocs::with_limit(100).order_by_score())
        .unwrap()
        .into_iter()
        .map(|(_, address)| {
            let doc: TantivyDocument = searcher.doc(address).unwrap();
            doc.get_first(index_field).unwrap().as_u64().unwrap() as usize // Since we don't have 4.3B objects
        })
        .collect()
}
