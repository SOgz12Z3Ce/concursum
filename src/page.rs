mod frame;
mod index;
mod object;
mod search;

use crate::data::cs::object::Key;
use crate::{app::Resource, data::Data};
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use maud::{DOCTYPE, Markup, html};
use std::{collections::HashMap, sync::Arc};

pub(crate) async fn index(State(resource): State<Arc<Resource>>) -> Html<String> {
    page(&resource.cs_data, index::content())
}

pub(crate) async fn object(
    State(resource): State<Arc<Resource>>,
    Path((group, id)): Path<(String, String)>,
) -> Html<String> {
    let key = Key {
        group: group,
        id: id,
    };
    let object = resource
        .cs_data
        .object(&key)
        .expect(&format!("not found: {}/{}", key.group, key.id));
    page(&resource.cs_data, object::content(&object))
}

pub(crate) async fn search(
    State(resource): State<Arc<Resource>>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let result = crate::search::search(&resource.cs_index, params);
    page(
        &resource.cs_data,
        search::content(&resource.cs_data, result),
    )
}

fn page(data: &Data, content: Markup) -> Html<String> {
    let page = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(data, None, None))
                    (content)
                }
                (frame::footer())
            }
        }
    };
    Html(page.into_string())
}
