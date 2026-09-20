use crate::{
    data::cs::{
        localization::{LocalizedObject, Summaries},
        text::{DeckAddition, RecipeAddition},
    },
    error::Error,
};
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
                        onerror=(format!("this.src='{fallback}';"));
                }

                (texts(&object.summary()?))
            }
        }
    })
}

pub(crate) fn texts(object: &Summaries) -> Markup {
    let zh_hans = object.zh_hans.as_ref().unwrap_or(&object.en_gb); // TODO: Remove this workaround.
    html! {
        @if let Some(label) = zh_hans.label {
            p class="content-field" {
                strong class="field-title" { "名称：" }
                (label)
            }
        }
        @if let Some(description) = zh_hans.description {
            p class="content-field" {
                strong class="field-title" { "描述：" }
                (description)
            }
        }
        @if let Some(slots) = &zh_hans.slots {
            p class="content-field" {
                strong class="field-title" { "卡槽：" }
                ul {
                    @for slot in slots {
                        li {
                            @if let Some(label) = slot.label {
                                span class="content-subfield" {
                                    strong class="subfield-title" { "名称：" }
                                    (label)
                                }
                            }
                            @if let Some(description) = slot.description {
                                span class="content-subfield" {
                                    strong class="subfield-title" { "描述：" }
                                    (description)
                                }
                            }
                        }
                    }
                }
            }
        }
        @if let Some(recipes) = &zh_hans.recipes {
            p class="content-field" {
                strong class="field-title" { "相关配方：" }
                ul {
                    @for recipe in recipes {
                        li {
                            @match recipe {
                                RecipeAddition::This { start_description } => {
                                    span class="content-subfield" {
                                        strong class="subfield-title" { "起始描述（自身）：" }
                                        (start_description)
                                    }
                                }
                                RecipeAddition::Other { label, description, start_description } => {
                                    @if let Some(label) = label {
                                        span class="content-subfield" {
                                            strong class="subfield-title" { "名称：" }
                                            (label)
                                        }
                                    }
                                    @if let Some(description) = description {
                                        span class="content-subfield" {
                                            strong class="subfield-title" { "描述：" }
                                            (description)
                                        }
                                    }
                                    @if let Some(start_description) = start_description {
                                        span class="content-subfield" {
                                            strong class="subfield-title" { "起始描述：" }
                                            (start_description)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        @if let Some(deck) = &zh_hans.deck {
            p class="content-field" {
                strong class="field-title" { "相关配方：" }
                ul {
                    li {
                        @match deck {
                            DeckAddition::This { draw_messages } => {
                                span class="content-subfield" {
                                    strong class="subfield-title" { "抽取时消息：" }
                                    @for (_, message) in draw_messages {
                                        (message)
                                    }
                                }
                            }
                            DeckAddition::Internal { label, description } => {
                                @if let Some(label) = label {
                                    span class="content-subfield" {
                                        strong class="subfield-title" { "名称：" }
                                        (label)
                                    }
                                }
                                @if let Some(description) = description {
                                    span class="content-subfield" {
                                        strong class="subfield-title" { "描述：" }
                                        (description)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
