use std::fs;
use pulldown_cmark::{Parser, Options};
mod template;
mod page;

use crate::page::Markdown;
fn main() {
    let content = fs::read_to_string("blog/test.md").unwrap();
    let test = page::Page::new(&content);
    println!("{}", test.render().into_string());
}
