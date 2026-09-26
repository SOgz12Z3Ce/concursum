use std::fmt::Debug;
use std::ops::Range;
use tantivy::{snippet::Snippet, tokenizer::TextAnalyzer};

pub(crate) trait GeneralSnippet: Debug {
    fn to_html(&self) -> String;
}

impl GeneralSnippet for Snippet {
    fn to_html(&self) -> String {
        self.to_html()
    }
}

#[derive(Debug)]
pub(crate) struct FuzzySnippet {
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
) -> (Vec<FuzzySnippet>, bool) {
    let mut snippets = Vec::new();
    let mut full_match = false;

    let phrase_tokens: Vec<&str> = phrases
        .iter()
        .flat_map(|phrase| tokenize(tokenizer.clone(), phrase))
        .collect();

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

        // Full match.
        if phrase_tokens.iter().any(|token| *token == token_content) {
            full_match = true;
        }

        // Highlight.
        if highlight(&phrase_tokens, token_content) {
            highlighted.push((token.offset_from - start_offset)..(token.offset_to - start_offset));
        }
        end_offset = token.offset_to;
    }
    if !highlighted.is_empty() {
        let fragment = (&text[start_offset..]).to_owned();
        snippets.push(FuzzySnippet::new(fragment, highlighted));
    }
    (snippets, full_match)
}

fn tokenize(mut tokenizer: TextAnalyzer, phrase: &str) -> Vec<&str> {
    let mut token_stream = tokenizer.token_stream(phrase);
    let mut phrases = Vec::new();
    while let Some(token) = token_stream.next() {
        let content = &phrase[token.offset_from..token.offset_to];
        phrases.push(content);
    }
    phrases
}

fn highlight(phrases: &Vec<&str>, token_content: &str) -> bool {
    for phrase in phrases {
        if phrase.chars().count() == 1 && *phrase == token_content {
            return true;
        }
        if phrase.chars().count() > 1 && strsim::osa_distance(*phrase, token_content) <= 1 {
            return true;
        }
    }
    return false;
}
