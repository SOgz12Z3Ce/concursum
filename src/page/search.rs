use crate::{data::cs::DataView, error::Error, search::SearchResult};
use maud::{Markup, PreEscaped, html};

pub(crate) fn content(
    data_view: &DataView,
    search_results: Vec<SearchResult>,
) -> Result<Markup, Error> {
    let results: Vec<(_, _)> = search_results
        .iter()
        .map(|SearchResult { index, snippets }| (&data_view.objects()[*index], snippets))
        .collect();

    Ok(html! {
        div id="content" {
            h2 id="content-title" {
                span id="content-title-prefix" { "Search: " }
                "Just wait..."
            }
            @for (object, snippets) in results {
                div class="search-result" {
                    h3 class="search-result-title" {
                        @let group = object.group()?;
                        @let id = object.id()?;
                        a href=(format!("cs/{}/{}", group, id)) {(id)}
                    }
                    ul {
                        @for snippet in snippets {
                            li {
                                (PreEscaped(snippet.to_html())) // Safety: Tantivy can escape snippets.
                            }
                        }
                    }
                }
            }
        }
    })
}
