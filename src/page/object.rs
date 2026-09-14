use crate::{data::cs::localization::LocalizedObject, error::Error};
use maud::{Markup, html};

pub(crate) fn content(object: &LocalizedObject) -> Result<Markup, Error> {
    let group = object.group()?;
    Ok(html! {
        div id="content" {
            div id="data-page" {
                h2 id="content-title" {
                    span id="content-title-prefix" { (group) ": " }
                    (object.id()?)
                    div class="copy-button data-copy ref" data-clipboard-text="Annoyance\nHere is a thorn in my side. I may yet find a way to remove it.\n\n" {
                        img class="ref-icon" alt="" src="/static/images/codex.png";
                        span class="ref-text ref-id" { "Copy" }
                    }
                }
                @let (icon, fallback) = object.icon()?;
                @let fallback = match fallback {
                    Some(icon) => format!("/static/images/cs/{icon}"),
                    None => String::from("/static/images/error.png"),
                };
                @if let Some(icon) = icon {
                    img class=(format!("content-image image-{} manifestation-empty", group))
                        alt="Icon"
                        src=(format!("/static/images/cs/{}", icon))
                        onerror=(fallback);
                }

                (object_fields(object))
            }
        }
    })
}

pub(crate) fn object_fields(object: &LocalizedObject) -> Markup {
    // TODO: New content page is comming!
    let properties = object.core().properties();
    html! {
        @for (key, value) in properties {
            p class="content-field" {
                strong class="field-title" { (key) "：" }
                (value)
            }
        }
    }
}
