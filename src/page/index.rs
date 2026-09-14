use maud::{Markup, html};

pub(crate) fn content() -> Markup {
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
