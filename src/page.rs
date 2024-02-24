use crate::template::body;
use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{Parser, Options, Event, html};

/// Renders a block of Markdown using `pulldown-cmark`.
pub struct Markdown<T>( pub T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        let mut unsafe_html = String::new();
        let parser = Parser::new(self.0.as_ref());
        html::push_html(&mut unsafe_html, parser);
        // Sanitize it with ammonia
        let safe_html = ammonia::clean(&unsafe_html);
        PreEscaped(safe_html)
    }
}

pub struct Page {
    title: String,
    content: PreEscaped<String>,
}

impl Page {
    pub fn new (page_path: &std::path::PathBuf) -> Self {
        let file_content = std::fs::read_to_string(page_path).unwrap();
        let mut title = String::new();
        let mut content = String::new();
        let parser = Parser::new_ext(&file_content, Options::ENABLE_HEADING_ATTRIBUTES);
        let parser = parser
            .map(|event| match event {
                Event::Text(ref text) => {
                    if title.is_empty() {
                        title = text.clone().into_string()
                    }
                    event
                },
                _ => event
        });
        html::push_html(&mut content, parser.into_iter());
        Self {
            title,
            content: PreEscaped(ammonia::clean(&content)),
        }
    }

    pub fn render(&self) -> Markup {
        body(Some(&self.title), &self.content)
    }
}
