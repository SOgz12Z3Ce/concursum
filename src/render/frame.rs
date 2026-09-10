use crate::{
    data::{DATA, Object},
    search::SearchResult,
};
use maud::{Markup, html};
use tantivy::snippet::Snippet;

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
                src="/static/images/icon.png";
            p id="tagline" {
                "欢迎来到聚点，漫宿的子午线所在之处，又称历史之室。在这里，你能看到漫宿发生的一切。"
            }
            p {
                "数据版本：2026.1.g.2"
            }
            div {
                p class="index-foot" {
                    "concursum 是公有领域软件，您可以自由地使用、分发、修改、商用此软件，无需服从任何条件。"
                    br;
                    strong { "Github 仓库：" }
                    a href="https://github.com/SOgz12Z3Ce/concursum" { "SOgz12Z3Ce/concursum" }
                }
                p class="index-foot" {
                    "基于 Lyrositor 的 "
                    a href="https://github.com/frangiclave/frangiclave-compendium" {
                        "Frangiclave Compendium"
                    }
                    " 以及 Igor Engel 的 "
                    a href="https://github.com/gt22/frangiclave" {
                        "Frangiclave Compendium (v2)"
                    }
                    " 修改而来。"
                }
                p id="donate" {
                    "您可以"
                    a href="https://ko-fi.com/lyrositor" { "为 Lyrositor 买杯咖啡" }
                    "。"
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
                img class=(format!("content-image image-{} manifestation-empty", object.group)) alt="Icon" src=(format!("/static/images/{}", object.icon()));
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

pub(crate) fn search(search_results: Vec<SearchResult>) -> Markup {
    let results: Vec<(Object, &Snippet)> = search_results
        .iter()
        .map(|result| (DATA.objects.index(result.index), &result.snippet))
        .collect();

    html! {
        div id="content" {
            h2 id="content-title" {
                span id="content-title-prefix" { "Search: " }
                "Just wait..."
            }
            @for (object, snippet) in results {
                div class="search-result" {
                    h3 class="search-result-title" {
                        a href=(format!("/{}/{}", object.group, object.id)) {(object.id)}
                    }
                    ul {
                        li {
                            (fragment(snippet))
                        }
                    }
                }
            }
        }
    }
}

fn fragment(snippet: &Snippet) -> Markup {
    let text = snippet.fragment();
    let mut slices = Vec::new();
    let mut cursor = 0;

    for range in snippet.highlighted() {
        if cursor < range.start {
            slices.push((text[cursor..range.start].to_owned(), false));
        }

        slices.push((text[range.start..range.end].to_owned(), true));
        cursor = range.end;
    }

    if cursor < text.len() {
        slices.push((text[cursor..].to_owned(), false));
    }

    html!(
        @for (content, highlighted) in slices {
            @if highlighted {
                strong { (content) }
            } @else {
                span { (content) }
            }
        }
    )
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
