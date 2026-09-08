mod frame;

use crate::data::DATA;
use axum::{extract::Path, response::Html};
use maud::{DOCTYPE, html};

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

pub(crate) async fn search() -> Html<String> {
    let content = html! {
        h1 {"search"}
    };
    Html(content.into_string())
}
