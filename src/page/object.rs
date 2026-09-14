use crate::data::cs::localization::LocalizedObject;
use maud::{Markup, html};

pub(crate) fn content(object: &LocalizedObject) -> Markup {
    html! {
        div id="content" {
            div id="data-page" {
                h2 id="content-title" {
                    span id="content-title-prefix" { (object.group()) ": " }
                    (object.id())
                    // div class="copy-button data-copy ref" data-clipboard-text="Annoyance\nHere is a thorn in my side. I may yet find a way to remove it.\n\n" {
                    //     img class="ref-icon" alt="" src="/static/images/codex.png";
                    //     span class="ref-text ref-id" { "Copy" }
                    // }
                }
                @if let Some(icon) = object.icon() {
                    img class=(format!("content-image image-{} manifestation-empty", object.group()))
                        alt="Icon"
                        src=(format!("/static/images/cs/{}", icon))
                        onerror="this.src=\"/static/images/error.png\"";
                }
                (object_fields(object))
            }
        }
    }
}

pub(crate) fn object_fields(object: &LocalizedObject) -> Markup {
    html! {
        @for (key, value) in &object.core.properties {
            p class="content-field" {
                strong class="field-title" { (key) "：" }
                (value)
            }
        }
    }
}
