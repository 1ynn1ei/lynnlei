use std::fs;
use maud::{Render};
mod template;
mod markdown;

use crate::markdown::Markdown;
fn main() {
    let content = fs::read_to_string("blog/test.md").unwrap();
    let markdown_content = Markdown(content);
    let markup = template::body(Some(String::from("test")),
        markdown_content.render()
    );
    println!("{}", markup.into_string());
}
