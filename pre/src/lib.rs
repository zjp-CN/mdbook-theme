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
        // if let Some(input) = ctx.config.get_preprocessor(self.name()) {
        //     if input.contains_key("blow-up") {
        //         // anyhow::bail!("Boom!!!");
        //         eprintln!("Boom!!!From preprocessor...");
        //     }
        //
        //     theme::config::process(input);
        // }

        ctx.config.get_preprocessor(self.name()).map(theme::config::process);

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool { renderer == "html" }
}
