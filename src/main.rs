use pulldown_cmark::{Parser, Options};
use std::io::prelude::*;
use page::Page;
mod template;
mod page;

use crate::page::Markdown;
fn main() {
    for entry in glob::glob("pages/**/*.md").expect("Failed to read pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path);
                let page = Page::new(&path);
                let output_path = path
                    .to_string_lossy()
                    .replace("pages", "dist")
                    .replace(".md", ".html");
                println!("{:?}", output_path);
                let mut file = std::fs::File::create(&output_path).unwrap();
                file.write_all(page.render().into_string().as_bytes()).unwrap();
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
