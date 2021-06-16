use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
#[macro_use]
extern crate serde_derive;

pub mod post;

pub struct Nop;

// impl Nop {
//     pub fn new() -> Nop { Nop }
// }

impl Preprocessor for Nop {
    fn name(&self) -> &str { "demo-pre" }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                // anyhow::bail!("Boom!!!");
                eprintln!("Boom!!!From preprocessor...");
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool { renderer != "not-supported" }
}
