use maud::{html, Markup, DOCTYPE};

pub fn body(htitle: Option<&String>, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        link rel="stylesheet" href="/styles.css";
        title {
            @if let Some(htitle) = htitle {
                "lynnlei"
                    "::"
                (htitle)
            }
        }
        main {
            (content)
        }
        footer {
            "Built on rust, Powered by maud, Cleaned by ammonia"
        }
    }
}


pub fn blog_index(data: Vec<(String, String)>) -> Markup {
    body(
        None,
        &html! {
            h1 { "Index" }
            ul.manifest {
                @for (title, link) in data {
                    li {
                        a href= { (link) } {
                            (title)
                        }
                    }
                }
            }
        }
    )
}
pub fn index() -> Markup {
    body(
        None,
        &html! {
            h4 { "hello! im lynn, yr (mostly) POSIX-compliant grl. proud lesbian.
                closeted user of electron apps. this is my slice of the internet, where
                    i talk about weird things im interested in." }
        }
    )
}
