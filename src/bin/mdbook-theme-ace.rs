// use mdbook::book::Chapter;
use mdbook::errors::Result;
use mdbook::renderer::RenderContext;
// use mdbook::BookItem;
use mdbook_theme::ace::Ace;
// use std::fs::{self, File};
use std::path::PathBuf;

fn main() -> Result<()> {
    let ctx = RenderContext::from_json(std::io::stdin())?;
    let mut cfg: Ace = ctx.config.get_deserialized_opt("output.theme-ace")?.unwrap();
    cfg.build_dir = ctx.config.build.build_dir.clone();
    cfg.theme_dir = ctx.config
                       .get("output.html.theme")
                       .map_or(PathBuf::from("theme"), |t| PathBuf::from(t.as_str().unwrap()));
    dbg!(cfg);

    // let book_path = ctx.config.build.build_dir;
    // println!("book/html exists: {}",
    // std::path::Path::new("/home/ubuntu/scripts/temp/mdbook-theme/examples/book/html").exists());
    // let html_path = ctx.root.join(book_path);
    // println!("{:?} and book.js exists: {}", html_path, html_path.join("book.js").exists());
    // dbg!(&ctx.destination);

    // let _ = fs::create_dir_all(&ctx.destination);
    // let path = ctx.destination.join("wordcounts.txt");
    // eprintln!("{:?}", path);
    // let mut f = if cfg.write_file { Some(File::create(path).unwrap()) } else { None };
    //
    // for item in ctx.book.iter() {
    //     if let BookItem::Chapter(ref ch) = *item {
    //         if cfg.ignores.contains(&ch.name) {
    //             continue;
    //         }
    //
    //         let num_words = count_words(ch);
    //         println!("{}: {}", ch.name, num_words);
    //         if let Some(f) = f.as_mut() {
    //             writeln!(f, "{}: {}", ch.name, num_words).unwrap();
    //         }
    //
    //         if cfg.deny_odds && num_words % 2 == 1 {
    //             eprintln!("{} has an odd number of words!", ch.name);
    //             // process::exit(1);
    //         }
    //     }
    // }
    Ok(())
}

// fn count_words(ch: &Chapter) -> usize { ch.content.split_whitespace().count() }
