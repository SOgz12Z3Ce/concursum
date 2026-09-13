mod frame;

use crate::{app::Resource, data::object::Key, search};
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use maud::{DOCTYPE, html};
use std::{collections::HashMap, sync::Arc};

pub(crate) async fn index(State(resource): State<Arc<Resource>>) -> Html<String> {
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(&resource.cs_data, None, None))
                    (frame::index_content())
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}

pub(crate) async fn page(
    State(resource): State<Arc<Resource>>,
    Path((group, id)): Path<(String, String)>,
) -> Html<String> {
    let key = Key {
        group: group.clone(),
        id: id.clone(),
    };
    let object = resource
        .cs_data
        .object(&key)
        .expect(&format!("not found: {}/{}", key.group, key.id));
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(&resource.cs_data, Some(group), Some(id)))
                    (frame::page_content(&object))
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}

pub(crate) async fn search(
    State(resource): State<Arc<Resource>>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let search_results = search::search(&resource.cs_index, params);
    let content = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(&resource.cs_data, None, None))
                    (frame::search(&resource.cs_data,search_results))
                }
                (frame::footer())
            }
        }
    };
    Html(content.into_string())
}
