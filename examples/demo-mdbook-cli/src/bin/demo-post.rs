use mdbook::book::Chapter;
use mdbook::errors::Result;
use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use std::fs::{self, File};
use std::io::{self, Write};
// use std::process;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WordcountConfig {
    pub ignores:    Vec<String>,
    pub deny_odds:  bool, // `deny-odds` in book.toml
    pub write_file: bool, // `deny-odds` in book.toml
}

fn main() -> Result<()> {
    // let mut stdin = io::stdin();
    // let ctx = RenderContext::from_json(&mut stdin)?;
    let ctx = RenderContext::from_json(io::stdin())?;
    let cfg: WordcountConfig = ctx.config.get_deserialized_opt("output.demo-post")?.unwrap();
    println!("{:?}", cfg);

    let book_path = ctx.config.build.build_dir;
    // println!("book/html exists: {}",
    // std::path::Path::new("/home/ubuntu/scripts/temp/mdbook-theme/examples/book/html").exists());
    let html_path = ctx.root.join(book_path);
    println!("{:?} and book.js exists: {}", html_path, html_path.join("book.js").exists());

    let _ = fs::create_dir_all(&ctx.destination);
    let path = ctx.destination.join("wordcounts.txt");
    eprintln!("{:?}", path);
    let mut f = if cfg.write_file { Some(File::create(path).unwrap()) } else { None };

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            let num_words = count_words(ch);
            println!("{}: {}", ch.name, num_words);
            if let Some(f) = f.as_mut() {
                writeln!(f, "{}: {}", ch.name, num_words).unwrap();
            }

            if cfg.deny_odds && num_words % 2 == 1 {
                eprintln!("{} has an odd number of words!", ch.name);
                // process::exit(1);
            }
        }
    }
    Ok(())
}

fn count_words(ch: &Chapter) -> usize { ch.content.split_whitespace().count() }
