mod frame;
mod index;
mod object;
mod search;

use crate::{
    app::Resource,
    data::cs::{DataView, object::Key},
    error::Error,
};
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use maud::{DOCTYPE, Markup, html};
use std::{collections::HashMap, sync::Arc};

static KEYWORDS_PARAM_NAME: &'static str = "keywords";

pub(crate) async fn index<'a>(
    State(resource): State<Arc<Resource<'a>>>,
) -> Result<Html<String>, Error> {
    page(&resource.data_view, index::content())
}

pub(crate) async fn object<'a>(
    State(resource): State<Arc<Resource<'a>>>,
    Path((group, id)): Path<(String, String)>,
) -> Result<Html<String>, Error> {
    let group = group.parse()?;
    let key = Key::new(group, &id);
    let object = resource
        .data_view
        .index(&key)
        .ok_or(Error::ObjectNotFound { group, id })?;
    Ok(page(&resource.data_view, object::content(&object)?)?)
}

pub(crate) async fn search<'a>(
    State(resource): State<Arc<Resource<'a>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, Error> {
    let Some(keywords) = params.get(KEYWORDS_PARAM_NAME) else {
        return Err(Error::EmptySearch);
    };
    let results = crate::search::search(&resource.search_engine, keywords)?;
    Ok(page(
        &resource.data_view,
        search::content(&resource.data_view, results)?,
    )?)
}

fn page(data: &DataView, content: Markup) -> Result<Html<String>, Error> {
    let page = html! {
        (DOCTYPE)
        html {
            (frame::head("concursum"))
            body {
                (frame::header())
                div id="container" {
                    (frame::sidebar(data, None)?)
                    (content)
                }
                (frame::footer())
            }
        }
    };
    Ok(Html(page.into_string()))
}
