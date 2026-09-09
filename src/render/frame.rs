use crate::data::{DATA, Object};
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
                img src="/static/images/knock.png" alt="knock";
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

pub(crate) fn sidebar() -> Markup {
    let groups = &DATA.groups;
    let group_files = &DATA.group_files;
    let files = &DATA.files;
    let file_objects = &DATA.file_objects;
    let objects = &DATA.objects;
    html! {
        div id="sidebar" {
            div id="sections" {
                @for group in groups {
                    div class="section-title" { (group) }
                    div class="section-list" {
                        @for &index in &group_files[group] {
                            div class="section-file" {
                                @let file = &files[index];
                                div class="section-file-title" { (&file) }
                                @for &index in &file_objects[file] {
                                    @let object = objects.index(index);
                                    @let group = &object.group;
                                    @let id = &object.id;
                                    a class="section-item" href=(format!("/{group}/{id}")) {(id)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn index_content() -> Markup {
    html! {
        div id="content" {
            img id="logo"
                src="/static/images/toolknockf.png"
                alt="toolknockf";
            p id="tagline" {
                "Welcome, friend, to the Frangiclave Compendium, the premier source of forbidden knowledge for the discerning occultist."
            }
            p {
                "The Frangiclave is an open-source repository for information about the contents of the game Cultist Simulator, as extracted from the game's files. Here you can browse the decks, elements, legacies, recipes and verbs included in the game."
                br;
                strong {
                    "Reuse is permitted if it follows the rules in the "
                    a href="https://weatherfactory.biz/sixth-history-community-licence/" {
                        "Sixth History License"
                    }
                }
            }
            p {
                "Data taken from: Cultist Simulator 2023.5.P.9"
            }
            div {
                p class="index-foot" {
                    strong { "Source: " }
                    a href="https://github.com/gt22/frangiclave" { "gt22/frangiclave" }
                }
                p class="index-foot" {
                    "Hosted by Frgm"
                    br;
                }
                p class="index-foot" {
                    "Based on the original "
                    a href="https://github.com/frangiclave/frangiclave-compendium" {
                        "Frangiclave Compendium"
                    }
                    " by Lyrositor"
                }
                p id="donate" {
                    "If you'd like to donate, feel free to "
                    a href="https://ko-fi.com/lyrositor" { "Buy Lyro a coffee" }
                    "."
                }
            }
        }
    }
}

pub(crate) fn page_content(object: &Object) -> Markup {
    html! {
        div id="content" {
            div id="data-page" {
                h2 id="content-title" {
                    span id="content-title-prefix" { (&object.group) ": " }
                    "just wait ..."
                    div class="copy-button data-copy ref" data-clipboard-text="Annoyance\nHere is a thorn in my side. I may yet find a way to remove it.\n\n" {
                        img class="ref-icon" alt="" src="/static/images/codex.png";
                        span class="ref-text ref-id" { "Copy" }
                    }
                }
                img class=(format!("content-image image-{} manifestation-empty", object.group)) alt="Icon" src=(format!("/static/images/cs/{}", object.icon()));
                (object_fields(object))
            }
        }
    }
}

pub(crate) fn object_fields(object: &Object) -> Markup {
    html! {
        @for (key, value) in object.content.as_object().unwrap() {
            p class="content-field" {
                strong class="field-title" { (key) "：" }
                (value)
            }
        }
    }
}

pub(crate) fn search(objects: Vec<Object>) -> Markup {
    html! {
        div id="content" {
            h2 id="content-title" {
                span id="content-title-prefix" { "Search: " }
                "Just wait..."
            }
            @for object in objects {
                div class="search-result" {
                    h3 class="search-result-title" {
                        (object.id)
                    }
                    ul {
                        li { "demo text" }
                    }
                }
            }
        }
    }
}

pub(crate) fn footer() -> Markup {
    html! {
        footer {
            "Cultist Simulator and Book of Hours is the sole property of Weather Factory. All rights reserved."
            br;
            "All game content on this website, including images and text, is used with permission."
            br;
            "Reuse is permitted if it follows the rules in the "
            a href="https://weatherfactory.biz/sixth-history-community-licence/"
              target="_blank" {
                "Sixth History License"
            }
        }
    }
}
