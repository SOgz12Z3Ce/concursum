use crate::{
    data::cs::{DataView, group::Group, object::Key},
    error::Error,
};
use maud::{Markup, html};
use syntect::{
    highlighting::ThemeSet,
    html::{self, ClassStyle},
};

pub(crate) fn head(title: &str) -> Markup {
    html! {
        head {
            // TODO: add OG meta labels
            // TODO: Such styles.css should be removed.
            link rel="stylesheet" type="text/css" id="styles" href="/static/styles/styles.css";
            link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Forum|Lato";
            // TODO: add themes
            script defer src="/static/scripts/sidebar.js" {}
            script defer src="/static/scripts/clipboard.min.js" {}
            title { (title) }
            style {
                (highlight())
                "pre { background-color: aliceblue; }"
            }
        }
    }
}

fn highlight() -> String {
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["InspiredGitHub"];
    html::css_for_theme_with_class_style(theme, ClassStyle::Spaced).unwrap()
}

pub(crate) fn header() -> Markup {
    html! {
        header {
            a href="/" {
                img src="/static/images/favicon.png" alt="knock";
                h1 { "concursum" }
            }
            form style="display: flex; align-items: center" id="search-box" action="/search" method="get" {
                label style="font-size: 1rem;" {
                    input type="checkbox" name="fuzzy" value="true" checked;
                    "模糊搜索"
                }
                input type="search"
                      title="Search"
                      name="keywords"
                      id="search-text"
                      value="";
                input type="submit" value="Search" id="search-submit";
            }
        }
    }
}

pub(crate) fn sidebar(data_view: &DataView, active_key: Option<&Key>) -> Result<Markup, Error> {
    Ok(html! {
        div id="sidebar" {
            div id="sections" {
                @for group in Group::ALL {
                    div class="section-title" { (group) }
                    div class=(format!("section-list{}", if active_key.is_some_and(|key| key.group() == group) {" section-list-opened"} else {""})) {
                        @for location in data_view.file_locations(group) {
                            div class="section-file" {
                                div class="section-file-title" { (location) }

                                @for object in data_view.location_objects(location) {
                                    @let group = object.group()?;
                                    @let id = object.id()?;
                                    @let this_key = object.key()?;

                                    @if active_key.is_some_and(|key| *key == this_key) {
                                        a id="section-item-active" class="section-item" href=(format!("/cs/{group}/{id}")) {(id)}
                                    } @else {
                                        a class="section-item" href=(format!("/cs/{group}/{id}")) {(id)}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub(crate) fn footer() -> Markup {
    html! {
        footer {
            "《密教模拟器》（" site { "Cultist Simulator" } "）和《司辰之书》（" site{ "Book of Hours" } "）为 Weather Factory 所有，保留所有权利。"
            br;
            "本站使用的所有游戏内容，包括图片与文字，均在 "
            a href="https://weatherfactory.biz/sixth-history-community-licence/"
              target="_blank" {
                site { "Sixth History Community Licence" }
            }
            " 的许可下使用。"
        }
    }
}
