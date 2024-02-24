
use std::io::prelude::*;
use page::Page;
mod template;
mod page;


fn main() {
    let mut links :Vec<(String, String)> = Vec::new();
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
                links.push((page.title, output_path.replace("dist", "")));
            },
            Err(e) => println!("{:?}", e),
        }
    }
    let mut file = std::fs::File::create("dist/index.html").unwrap();
    file.write_all(template::index(links).into_string().as_bytes()).unwrap();

    std::fs::copy("styles.css", "dist/styles.css").unwrap();
}
