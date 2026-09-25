use std::fmt::Debug;
use std::ops::Range;
use tantivy::{snippet::Snippet, tokenizer::TextAnalyzer};

pub(crate) trait GeneralSnippet: Debug {
    fn is_empty(&self) -> bool;

    fn to_html(&self) -> String;
}

impl GeneralSnippet for Snippet {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn to_html(&self) -> String {
        self.to_html()
    }
}

#[derive(Debug)]
struct FuzzySnippet {
    fragment: String,
    highlighted: Vec<Range<usize>>,
}

impl FuzzySnippet {
    fn new(fragment: String, highlighted: Vec<Range<usize>>) -> Self {
        Self {
            fragment,
            highlighted,
        }
    }
}

impl GeneralSnippet for FuzzySnippet {
    fn is_empty(&self) -> bool {
        self.fragment.len() == 0
    }
    fn to_html(&self) -> String {
        let mut html = String::new();
        let mut start_from: usize = 0;

        for item in tantivy::snippet::collapse_overlapped_ranges(&self.highlighted) {
            html.push_str(&htmlescape::encode_minimal(
                &self.fragment[start_from..item.start],
            ));
            html.push_str("<b>");
            html.push_str(&htmlescape::encode_minimal(&self.fragment[item.clone()]));
            html.push_str("</b>");
            start_from = item.end;
        }
        html.push_str(&htmlescape::encode_minimal(
            &self.fragment[start_from..self.fragment.len()],
        ));
        html
    }
}

pub(crate) fn fuzzy_snippet(
    mut tokenizer: TextAnalyzer,
    phrases: &Vec<&str>,
    text: &str,
) -> (Vec<Box<dyn GeneralSnippet>>, bool) {
    let mut full_match = false;
    let mut snippets = Vec::new();
    let mut token_stream = tokenizer.token_stream(text);
    let mut start_offset = 0;
    let mut end_offset = 0;
    let mut highlighted = Vec::new();
    while let Some(token) = token_stream.next() {
        if token.offset_to - start_offset > 150 {
            if !highlighted.is_empty() {
                let fragment = (&text[start_offset..end_offset]).to_owned();
                snippets.push(FuzzySnippet::new(fragment, highlighted));
                highlighted = Vec::new();
            }
            start_offset = token.offset_from;
        }

        let token_content = &text[token.offset_from..token.offset_to];
        if phrases.iter().any(|phrase| *phrase == token_content) {
            full_match = true;
        }
        if phrases
            .iter()
            .any(|phrase| strsim::osa_distance(token_content, phrase) <= 1)
        {
            highlighted.push((token.offset_from - start_offset)..(token.offset_to - start_offset));
        }
        end_offset = token.offset_to;
    }
    if !highlighted.is_empty() {
        let fragment = (&text[start_offset..]).to_owned();
        snippets.push(FuzzySnippet::new(fragment, highlighted));
    }
    let snippets = snippets
        .into_iter()
        .map(|snippet| {
            let snippet: Box<dyn GeneralSnippet> = Box::new(snippet);
            snippet
        })
        .collect();
    (snippets, full_match)
}
