mod frame;

use crate::{
    data::{DATA, Object},
    search,
};
use axum::{
    extract::{Path, Query},
    response::Html,
};
use maud::{DOCTYPE, html};
use std::collections::HashMap;

pub(crate) async fn index() -> Html<String> {
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar())
                    (frame::index_content())
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}

pub(crate) async fn page(Path((group, id)): Path<(String, String)>) -> Html<String> {
    let object = DATA.object(&group, &id);
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar())
                    (frame::page_content(&object))
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}

pub(crate) async fn search(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let object_indices = search::search(params);
    let objects: Vec<Object> = object_indices
        .iter()
        .map(|i| DATA.objects.index(*i))
        .collect();
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar())
                    (frame::search(objects))
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}
