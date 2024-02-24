use maud::{html, Markup, DOCTYPE};

pub fn body(htitle: Option<&String>, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title {
            @if let Some(htitle) = htitle {
                (htitle)
                " 〜 "
            }
            "lynnlei"
        }
        link rel="stylesheet" href="/styles.css";
        main {
            (content)
        }
    }
}

pub fn index(data: Vec<(String, String)>) -> Markup {
    body(
        None,
        &html! {
            h1 { "Blog" }
            ul.manifest {
                @for (title, link) in data {
                    a href= { (link) } {
                        (title)
                    }
                }
            }
        }
    )
}
