#![allow(unused)]
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod theme;
pub struct Nop;

// impl Nop {
//     pub fn new() -> Nop { Nop }
// }

impl Preprocessor for Nop {
    fn name(&self) -> &str { "theme" }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        // if let Some(input) = ctx.config.get_preprocessor(self.name()) {
        //     if input.contains_key("blow-up") {
        //         // anyhow::bail!("Boom!!!");
        //         eprintln!("Boom!!!From preprocessor...");
        //     }
        //
        //     theme::config::process(input);
        // }

        let dir = ctx.config.get("output.html.theme").map_or("theme", |s| s.as_str().unwrap());
        if let Some(theme) = ctx.config.get_preprocessor(self.name()) {
            theme::config::process(theme, dir);
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool { renderer == "html" }
}
