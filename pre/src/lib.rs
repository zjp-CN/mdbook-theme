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
    fn name(&self) -> &str { "theme-pre" }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                // anyhow::bail!("Boom!!!");
                eprintln!("Boom!!!From preprocessor...");
            }
        }

        let theme_config = theme::config::themes();

        dbg!(&theme_config);

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool { renderer != "not-supported" }
}
