mod frame;

use crate::{
    data::{DATA, object::Key},
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
                    (frame::sidebar(None, None))
                    (frame::index_content())
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}

pub(crate) async fn page(Path((group, id)): Path<(String, String)>) -> Html<String> {
    let key = Key {
        group: group.clone(),
        id: id.clone(),
    };
    let object = DATA
        .object(&key)
        .expect(&format!("not found: {}/{}", key.group, key.id));
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(Some(group), Some(id)))
                    (frame::page_content(&object))
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}

pub(crate) async fn search(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let search_results = search::search(params);
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(None, None))
                    (frame::search(search_results))
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}
