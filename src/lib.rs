use mdbook::{
    book::Book,
    errors,
    preprocess::{Preprocessor, PreprocessorContext},
    Config,
};
use std::{
    path::{Path, PathBuf},
    result,
};

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
    DirNotRemoved,
    DirNotRead,
    AceNotFound,
    MdbookNotParsed,
    DeserializedFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
// pub mod error;

pub struct PreTheme;

/// absoulte path to theme dir:
/// if `output.html.theme` is not provided, it defaults to `theme/`
pub fn theme_dir(root: &Path, config: &Config) -> PathBuf {
    let theme_dir_ = config
        .get("output.html.theme")
        .and_then(|v| v.as_str())
        .unwrap_or("theme");
    root.join(theme_dir_)
}

impl Preprocessor for PreTheme {
    fn name(&self) -> &str {
        "theme"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> result::Result<Book, errors::Error> {
        let dir = theme_dir(&ctx.root, &ctx.config);
        if let Some(theme) = ctx.config.get_preprocessor(self.name()) {
            theme::config::run(theme, dir);
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}
