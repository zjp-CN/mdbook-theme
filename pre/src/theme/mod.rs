use mdbook::theme::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::path::PathBuf;

use crate::error::{Error, Result};

pub mod config;

/// All cssfiles to be modified.
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CssFile { 
    Variables, General, Chrome, Index, PagetocJs, Invalid,
    PagetocCss, Pagetoc, Custom(&'static str) 
}

#[rustfmt::skip]
macro_rules! default {
    ($idt:ident, $e1:expr) => { (CssFile::$idt, $e1) };
    ($idt:ident, $e1:expr, $e2:expr) => { (CssFile::$idt, Item($e1), Value($e2)) };
}

// TODO: add more
#[rustfmt::skip]
pub static CSSFILES: &[(CssFile, &'static str)] = 
  &[default!(Variables, "css/variables.css"),
    default!(Index,     "index.hbs"),
    default!(Invalid,   "")];

impl CssFile {
    pub fn filename(&self) -> &'static str {
        match self {
            CssFile::Custom(filename) => filename,
            _ => {
                let index = CSSFILES.binary_search_by_key(self, |&(a, b)| a).unwrap();
                dbg!(CSSFILES[index].1)
            }
        }
    }

    pub fn variant(filename: &str) -> Self {
        let index = CSSFILES.binary_search_by_key(&filename, |&(a, b)| b).unwrap();
        CSSFILES[index].0
    }
}

/// 1. supported items (config args)
/// 2. item of `preprocessor.theme-pre` table in book.toml
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Item<'a>(&'a str);

/// useful when looking up in `HashMap<&Item, _>` just via `HashMap<&str, _>`
impl<'a> Borrow<str> for Item<'a> {
    fn borrow(&self) -> &str { self.0 }
}

impl<'a> Item<'a> {
    pub fn get(&self) -> &str { self.0 }
}

/// by default or specified by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value<'b>(&'b str);

impl<'b> Value<'b> {
    pub fn get(&self) -> &str { self.0 }
}

/// configs ready to go
#[derive(Debug, Clone)]
pub struct Ready<'a, 'b>(Vec<(Item<'a>, Value<'b>)>);

impl<'a, 'b> Default for Ready<'a, 'b> {
    fn default() -> Self { Self(vec![]) }
}

/// get `Ready` by using `iter.collect()`
impl<'a, 'b> FromIterator<(Item<'a>, Value<'b>)> for Ready<'a, 'b> {
    fn from_iter<I: IntoIterator<Item = (Item<'a>, Value<'b>)>>(iter: I) -> Self {
        let mut r = Self::default();
        for i in iter {
            r.0.push(i);
        }
        r
    }
}

/// yield default config or merge configs
impl<'a, 'b> Ready<'a, 'b> {
    /// TODO: add more default config
    /// to get a default config from a specific cssfile
    pub fn get_defualt(css: CssFile) -> Self {
        match css {
            c @ CssFile::Variables => Ready::from(c),
            _ => Self::default(),
        }
    }

    /// help to simplify `.get()`
    fn from(css: CssFile) -> Self {
        DEFAULT.iter().filter(|(c, _, _)| *c == css).map(|(_, i, v)| (*i, *v)).collect()
    }

    // fn add(&mut self, elem: (Item, Value)) { self.0.push(elem); }

    pub fn item_value(&self) -> &Vec<(Item, Value)> { &self.0 }
}

#[derive(Clone)]
pub struct Content(String);

impl Default for Content {
    fn default() -> Self { Self(String::from("")) }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Content(...)") }
}

impl Content {
    /// TODO: add more contents
    pub fn from(cssfile: CssFile) -> Self {
        use std::str;
        match cssfile {
            CssFile::Variables => Content(String::from(str::from_utf8(VARIABLES_CSS).unwrap())),
            CssFile::Custom(filename) => Content::from_file(filename),
            _ => Content::default(),
        }
    }

    fn from_file(filename: &str) -> Self {
        use std::io::Read;
        let mut s = String::new();
        let path = format!("theme/{}", filename);
        dbg!(&path);
        std::fs::File::open(path).unwrap().read_to_string(&mut s).unwrap();
        Content(s)
    }

    /// for viewing the content
    pub fn get(&self) -> &str { &self.0 }

    /// for modifying the content
    pub fn get_mut(&mut self) -> &mut String { &mut self.0 }

    /// hypothesis: `item: value;`
    /// better to use `regex`, but for now I'm not ready :(
    fn find(&self, pat: &str) -> Result<(usize, usize)> {
        let text = self.get();
        let p1 = text.find(pat).ok_or(Error::StrNotFound)? + pat.len() + 2;
        let p2 = p1 + text[p1..].find(';').ok_or(Error::StrNotFound)?;
        // dbg!(&text[p1..p2]);
        Ok((p1, p2))
    }

    /// update the content
    fn replace(&mut self, pat: &str, sub: &str) -> Result<()> {
        let (p1, p2) = self.find(pat)?;
        self.get_mut().replace_range(p1..p2, sub);
        eprintln!("\n{}", &self.get()[p1 - 20..p2 + 10]);
        Ok(())
    }

    /// Insert content, and need two str to find.
    /// The first is to find backwards;
    /// the second is to locate the inserted space right one char ahead.
    fn insert(&mut self, insert: &str, find1: &str, find2: &str) -> Result<()> {
        let text = self.get();
        let mut pos = text.find(find1).ok_or(Error::StrNotFound)?;
        pos = pos + text[pos..].find(find2).ok_or(Error::StrNotFound)? - 1;
        self.get_mut().replace_range(pos..pos + 1, insert);
        // eprintln!("\n{}", &self.get()[pos - 20..pos + 20]);
        Ok(())
    }

    #[rustfmt::skip]
    fn variables(&mut self, pat: &str, sub: &str) {
        if pat == "mobile-content-max-width" {
            let content = format!(
"\n@media only screen and (max-width:1439px) {{
 :root{{
    --content-max-width: {};
  }}
}}\n\n", sub);
            self.insert(&content, "}", "/* Themes */");
        } else if self.replace(pat, sub).is_err() {
                self.insert(&format!("\n    --{}: {};\n", pat, sub), ":root", "}\n").unwrap();
            }
    }
}

// TODO: add more static variables, and may remove the needless `Value` and tuples
pub static DEFAULT: &[(CssFile, Item, Value)] =
    &[default!(Variables, "sidebar-width", "140px"),
      default!(Variables, "page-padding", "15px"),
      default!(Variables, "content-max-width", "82%"),
      default!(Variables, "menu-bar-height", "40px"),
      default!(Variables, "pagetoc-width", "13%"),
      default!(Variables, "pagetoc-fontsize", "14.5px"),
      default!(Variables, "mobile-content-max-width", "98%")];

static INDEX_PAGETOC: &str = r#"
<!-- Page table of contents -->\
<div class="sidetoc"><nav class="pagetoc"></nav></div>
"#;

#[derive(Clone)]
pub struct Theme<'a, 'b> {
    pub cssfile: CssFile,
    pub content: Content, // ultimate str to be processed
    pub ready:   Ready<'a, 'b>, /* need a func to cover the default values according to user's
                           * config */
}

#[rustfmt::skip]
impl<'a, 'b> Default for Theme<'a, 'b> {
    fn default() -> Self {
        Self { cssfile: CssFile::Custom(""), content: Content::default(), ready: Ready::default() }
    }
}

impl<'a, 'b> fmt::Debug for Theme<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Theme")
         .field("cssfile", &self.cssfile)
         .field("content", &self.content)
         .field("ready", &self.ready.0.len())
         .finish()
    }
}

impl<'a, 'b> Theme<'a, 'b> {
    #[rustfmt::skip]
    pub fn from(cssfile: CssFile, ready: Ready<'a, 'b>) -> Self {
        Self { cssfile, ready, content: Content::default() }
    }

    pub fn process(mut self) -> Self { self.cssfile().content().write_theme_file() }

    /// Give a default or custom virtual css file marked to help content processing.
    fn cssfile(mut self) -> Self {
        let filename = self.cssfile.filename();
        // TODO: make path a field in `Theme`
        if std::path::Path::new("theme").join(filename).exists() {
            self.cssfile = CssFile::Custom(filename);
        }
        self
    }

    /// The **final** content to be written into `theme` dir.
    /// An empty content means not having processed the content.
    fn content(mut self) -> Self {
        self.content = Content::from(self.cssfile);
        self.content_process(None);
        self
    }

    /// process contents of different files
    fn content_process(&mut self, filename: Option<&str>) {
        let cssfile = filename.map_or_else(|| self.cssfile.clone(), |f| CssFile::variant(f));
        dbg!(&cssfile);
        match cssfile {
            CssFile::Variables => self.process_variables(),
            // TODO: add more branches
            CssFile::Custom(f) => self.content_process(Some(f)),
            _ => (),
        }
    }

    /// create a css file on demand
    fn write_theme_file(self) -> Self {
        use std::fs::write;
        write(std::path::Path::new("theme").join(self.cssfile.filename()),
              self.content.get().as_bytes()).unwrap();
        dbg!(&self);
        self
    }

    /// Swich to another cssfile and process its content, which can repeat.
    fn ready(mut self, cssfile: CssFile) -> Self {
        self.cssfile = cssfile;
        self.ready = Ready::get_defualt(cssfile);
        self.process()
    }

    /// When `pagetoc = true` , a bunch of files need to change; when NOT true, do nothing.
    pub fn pagetoc(self) {
        if self.cssfile == CssFile::Pagetoc {
            self.ready(CssFile::Variables);
            // .ready(CssFile::Index)
            // .ready(CssFile::General)
            // .ready(CssFile::Chrome)
        }
    }

    /// update content in `variables.css`
    fn process_variables(&mut self) {
        for (item, value) in self.ready.item_value() {
            self.content.variables(item.get(), value.get());
        }
    }

    /// create the dirs on demand
    pub(self) fn create_theme_dirs() -> Result<()> {
        std::fs::create_dir_all("theme/css").map_err(|_| Error::DirNotCreated)?;
        Ok(())
    }
}
