pub(crate) mod exact;
pub(crate) mod fuzzy;
mod snippet;

use crate::{data::cs::DataView, error::Error, search::snippet::GeneralSnippet};
use tantivy::{
    Index, TantivyDocument,
    schema::{FAST, Field, IndexRecordOption, STORED, Schema, TextFieldIndexing, TextOptions},
};
use tantivy_jieba::JiebaTokenizer;

pub(crate) const TOKENIZER: &'static str = "jieba";
// static NGRAM_TOKENIZER_NAME: &'static str = "1_2-gram";
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
                    .set_tokenizer(TOKENIZER)
                    // .set_tokenizer(NGRAM_TOKENIZER_NAME)
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
        TOKENIZER,
        JiebaTokenizer::default(),
        // NGRAM_TOKENIZER_NAME,
        // NgramTokenizer::new(2, 2, false).unwrap(),
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
