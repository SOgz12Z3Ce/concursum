use crate::{
    data::{Data, localization::LocalizedObject},
    search::SearchResult,
};
use maud::{Markup, html};
use tantivy::snippet::Snippet;

pub(crate) fn content(cs_data: &Data, search_results: Vec<SearchResult>) -> Markup {
    let results: Vec<(&LocalizedObject, &Snippet)> = search_results
        .iter()
        .map(|result| {
            (
                cs_data.localized_objects.get(result.index).unwrap(),
                &result.snippet,
            )
        })
        .collect();

    html! {
        div id="content" {
            h2 id="content-title" {
                span id="content-title-prefix" { "Search: " }
                "Just wait..."
            }
            @for (object, snippet) in results {
                div class="search-result" {
                    h3 class="search-result-title" {
                        a href=(format!("cs/{}/{}", object.group(), object.id())) {(object.id())}
                    }
                    ul {
                        li {
                            (fragment(snippet))
                        }
                    }
                }
            }
        }
    }
}

fn fragment(snippet: &Snippet) -> Markup {
    let text = snippet.fragment();
    let mut slices = Vec::new();
    let mut cursor = 0;

    for range in snippet.highlighted() {
        if cursor < range.start {
            slices.push((text[cursor..range.start].to_owned(), false));
        }

        slices.push((text[range.start..range.end].to_owned(), true));
        cursor = range.end;
    }

    if cursor < text.len() {
        slices.push((text[cursor..].to_owned(), false));
    }

    html!(
        @for (content, highlighted) in slices {
            @if highlighted {
                strong { (content) }
            } @else {
                span { (content) }
            }
        }
    )
}
