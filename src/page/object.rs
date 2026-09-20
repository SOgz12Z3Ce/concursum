use crate::{
    data::cs::{
        localization::{LocalizedObject, Summaries}, text::{DeckAddition, RecipeAddition, SlotAddition},
    }, error::Error,
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
        (zh_hans.label.render("名称"))
        (zh_hans.description.render("描述"))
        @if let Some(slots) = &zh_hans.slots {
            p class="content-field" {
                strong class="field-title" { "卡槽：" }
                ul {
                    @for slot in slots {
                        li {
                            (slot.label.render("名称"))
                            (slot.description.render("描述"))
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
                strong class="field-title" { "卡组：" }
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
        @if let Some(legacy) = &zh_hans.legacy {
            p class="content-field" {
                strong class="field-title" { "职业：" }
                ul {
                    li {
                        {
                            span class="content-subfield" {
                                strong class="subfield-title" { "开始时描述：" }
                                (legacy.start_description)
                            }
                        }
                    }
                }
            }
        }
    }
}

trait Renderer {
    fn render(&self, field: &str) -> Markup;
}

impl Renderer for &str {
    fn render(&self, field: &str) -> Markup {
        html! {
            p class="content-field" {
                strong class="field-title" { (format!("{field}：")) }
                (self)
            }
        }
    }
}

impl<T: Renderer> Renderer for Option<T> {
    fn render(&self, field: &str) -> Markup {
        let Some(text) = self else {
            return html! {};
        };
        text.render(field)
    }
}

impl<'a> Renderer for SlotAddition<'a> {
    fn render(&self, field: &str) -> Markup {
        todo!()
    }
}
