#![allow(unused)]
use mdbook::book::Book;
use mdbook::errors;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use std::result;

use crate::theme::{CssFile, Item, Value};

/// Generate some default static value. This macro is not public.
macro_rules! default {
    ($idt:ident, $e1:expr) => { (CssFile::$idt, $e1) };
    ($idt:ident, $e1:expr, $e2:expr) => { (CssFile::$idt, Item($e1), Value($e2)) };
    ($($e1:expr, $idt:ident);*) => {
        $(pub static $idt: &[u8] = include_bytes!($e1);)*
        pub static ACE_DEFAULT: &[(&str, &[u8])] = &[$(($e1, $idt),)*];
    };
}

pub mod ace;
pub mod theme;

#[derive(Debug)]
pub enum Error {
    StrNotFound,
    FileNotFound,
    FileNotCreated,
    FileNotWritten,
    DirNotCreated,
    AceNotFound,
    MdbookNotParsed,
    DeserializedFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
// pub mod error;

pub struct PreTheme;

impl Preprocessor for PreTheme {
    fn name(&self) -> &str { "theme" }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> result::Result<Book, errors::Error> {
        let dir = ctx.config.get("output.html.theme").map_or("theme", |s| s.as_str().unwrap());
        if let Some(theme) = ctx.config.get_preprocessor(self.name()) {
            theme::config::process(theme, dir);
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool { renderer == "html" }
}
