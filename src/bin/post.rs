use mdbook::book::Chapter;
use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WordcountConfig {
    pub ignores:   Vec<String>,
    pub deny_odds: bool, // `deny-odds` in book.toml
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let cfg: WordcountConfig =
        ctx.config.get_deserialized_opt("output.theme-post").unwrap().unwrap();
    println!("{:?}", cfg);

    let _ = fs::create_dir_all(&ctx.destination);
    let mut f = File::create(ctx.destination.join("wordcounts.txt")).unwrap();

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            let num_words = count_words(ch);
            println!("{}: {}", ch.name, num_words);
            writeln!(f, "{}: {}", ch.name, num_words).unwrap();

            if cfg.deny_odds && num_words % 2 == 1 {
                eprintln!("{} has an odd number of words!", ch.name);
                process::exit(1);
            }
        }
    }
}

fn count_words(ch: &Chapter) -> usize { ch.content.split_whitespace().count() }
