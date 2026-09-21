use std::collections::HashMap;

use crate::{
    data::cs::{
        localization::{LocalizedObject, Summaries},
        text::{DeckAddition, LegacyAddition, RecipeAddition, SlotAddition},
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
        (field(("名称", zh_hans.label)))
        (field(("描述", zh_hans.description)))
        (field(("卡槽", &zh_hans.slots)))
        (field(("相关配方", &zh_hans.recipes)))
        (field(("卡组", &zh_hans.deck)))
        (field(("职业", &zh_hans.legacy)))
    }
}

fn field<T: Renderer>(renderer: T) -> Markup {
    match renderer.render() {
        Some(content) => html! {
            p class="content-field" {
                (content)
            }
        },
        None => html! {},
    }
}

fn sub_field<T: Renderer>(renderer: T) -> Markup {
    match renderer.render() {
        Some(content) => html! {
            li class="content-field" {
                (content)
            }
        },
        None => html! {},
    }
}

trait Renderer {
    fn render(&self) -> Option<Markup>;
}

impl<'a, T: ?Sized + Renderer> Renderer for &'a T {
    fn render(&self) -> Option<Markup> {
        (**self).render()
    }
}

impl<T: Renderer> Renderer for (&str, T) {
    fn render(&self) -> Option<Markup> {
        let (name, renderer) = self;
        renderer.render().map(|text| {
            html! {
                strong class="field-title" { (format!("{name}：")) }
                (text)
            }
        })
    }
}

impl<T: Renderer> Renderer for Option<T> {
    fn render(&self) -> Option<Markup> {
        match self {
            Some(renderer) => renderer.render(),
            None => None,
        }
    }
}

impl<T: Renderer> Renderer for Vec<T> {
    fn render(&self) -> Option<Markup> {
        let texts: Vec<Option<Markup>> = self.iter().map(|renderer| renderer.render()).collect();
        if texts.iter().all(|text| text.is_none()) {
            return None;
        }
        let texts = texts
            .into_iter()
            .map(|text| text.unwrap_or(html! {"（无文本）"}));
        Some(html! {
            ul {
                @for text in texts {
                    li {
                        (text)
                    }
                }
            }
        })
    }
}

impl Renderer for &str {
    fn render(&self) -> Option<Markup> {
        Some(html! {(self)})
    }
}

impl Renderer for HashMap<&String, &str> {
    fn render(&self) -> Option<Markup> {
        Some(html! {
            ul {
                @for (key, value) in self {
                    li {
                        (format!("{key} -> {value}"))
                    }
                }
            }
        })
    }
}

impl<'a> Renderer for SlotAddition<'a> {
    fn render(&self) -> Option<Markup> {
        match self {
            Self {
                label: None,
                description: None,
            } => None,
            Self { label, description } => Some(html! {
                ul {
                    (sub_field(("名称", label)))
                    (sub_field(("描述", description)))
                }
            }),
        }
    }
}

impl<'a> Renderer for RecipeAddition<'a> {
    fn render(&self) -> Option<Markup> {
        match self {
            RecipeAddition::This { start_description } => Some(html! {
                ul {
                    (sub_field(("（自身）开始时描述", start_description)))
                }
            }),
            RecipeAddition::Other {
                label: None,
                description: None,
                start_description: None,
            } => None,
            RecipeAddition::Other {
                label,
                description,
                start_description,
            } => Some(html! {
                ul {
                    (sub_field(("名称", label)))
                    (sub_field(("描述", description)))
                    (sub_field(("开始时描述", start_description)))
                }
            }),
        }
    }
}

impl<'a> Renderer for DeckAddition<'a> {
    fn render(&self) -> Option<Markup> {
        match self {
            DeckAddition::This { draw_messages } => {
                Some(html! {
                    ul {
                        (sub_field(("抽取消息", draw_messages)))
                    }
                })
            }
            DeckAddition::Internal {
                label: None,
                description: None,
            } => None,
            DeckAddition::Internal { label, description } => Some(html! {
                ul {
                    (sub_field(("名称", label)))
                    (sub_field(("描述", description)))
                }
            }),
        }
    }
}

impl<'a> Renderer for LegacyAddition<'a> {
    fn render(&self) -> Option<Markup> {
        Some(html! {
            ul {
                (sub_field(("开始时描述", self.start_description)))
            }
        })
    }
}
