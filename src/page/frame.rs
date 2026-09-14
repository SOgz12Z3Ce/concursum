use crate::data::Data;
use maud::{Markup, html};

pub(crate) fn head(title: &str) -> Markup {
    html! {
        head {
            // TODO: add OG meta labels
            // TODO: add icon
            link rel="stylesheet" type="text/css" id="styles" href="/static/styles/styles.css";
            link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Forum|Lato";
            // TODO: add themes
            script defer src="/static/scripts/sidebar.js" {}
            script defer src="/static/scripts/clipboard.min.js" {}
            title { (title) }
        }
    }
}

pub(crate) fn header() -> Markup {
    html! {
        header {
            a href="/" {
                img src="/static/images/favicon.png" alt="knock";
                h1 { "concursum" }
            }
            form id="search-box" action="/search" method="get" {
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

pub(crate) fn sidebar(
    cs_data: &Data,
    expanded_group: Option<String>,
    active_id: Option<String>,
) -> Markup {
    let groups = &cs_data.groups;
    let group_files = &cs_data.group_files;
    let files = &cs_data.files;
    let file_objects = &cs_data.file_objects;
    let objects = &cs_data.localized_objects;
    html! {
        div id="sidebar" {
            div id="sections" {
                @for group in groups {
                    div class="section-title" { (group) }
                    div class=(format!("section-list{}", if expanded_group.as_ref().is_some_and(|g| g == group) {" section-list-opened"} else {""})) {
                        @for &index in &group_files[group] {
                            div class="section-file" {
                                @let file = &files[index];
                                div class="section-file-title" { (&file) }

                                @for &index in &file_objects[file] {
                                    @let object = objects.get(index).unwrap();
                                    @let group = object.group();
                                    @let id = object.id();
                                    @if active_id.as_ref().is_some_and(|i| i == id) {
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
    }
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
