use mdbook::theme::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::str;

use crate::error::{Error, Result};

pub mod config;

/// TODO: consider to remove tuples
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssFile { Variables, General, Chrome, Index, PagetocJs, PagetocCss, Custom, Pagetoc, }
// `Pagetoc` is not only single file

impl CssFile {
    fn filename(&self) -> &str {
        match self {
            CssFile::Variables => "css/variables.css",
            _ => "temp.css",
        }
    }
}

/// TODO: may not need this (use `match` instead)
// impl Borrow<[u8]> for CssFile {
//     fn borrow(&self) -> &'static [u8] {
//         match self {
//             CssFile::Pagetoc => &[],
//             CssFile::Custom(x) => &[],
//             CssFile::Variables(x) => x,
//             CssFile::General(x) => x,
//             CssFile::Chrome(x) => x,
//             CssFile::Index(x) => x,
//             CssFile::PagetocJs(x) => x,
//             CssFile::PagetocCss(x) => x,
//         }
//     }
// }

/// 1. supported items (config args)
/// 2. item of `preprocessor.theme-pre` table in book.toml
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Item(&'static str);

/// useful when looking up in `HashMap<&Item, _>`
impl Borrow<str> for &Item {
    fn borrow(&self) -> &str { self.0 }
}

impl Item {
    fn get(&self) -> &str { self.0 }
}

/// by default or specified by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Value(&'static str);

impl Value {
    fn get(&self) -> &str { self.0 }
}

#[derive(Debug, Clone)]
pub struct Ready(Vec<(Item, Value)>);

impl Default for Ready {
    fn default() -> Self { Self(vec![]) }
}

/// get `Ready` by using `iter.collect
impl FromIterator<(Item, Value)> for Ready {
    fn from_iter<I: IntoIterator<Item = (Item, Value)>>(iter: I) -> Self {
        let mut r = Self::default();
        for i in iter {
            r.add(i);
        }
        r
    }
}

/// yield default config or merge configs
impl Ready {
    /// TODO: add more default config
    fn get(css: CssFile) -> Self {
        match css {
            c @ CssFile::Variables => Ready::from(c),
            _ => Self::default(),
        }
    }

    fn from(css: CssFile) -> Self {
        DEFAULT.into_iter()
               .filter(|(c, _, _)| *c == css)
               .map(|(_, i, v)| (*i, *v))
               .collect()
    }

    /// `user_config` can be derive from `iter::collect`
    fn merge(&mut self, css: CssFile) -> Self {
        let mut default = Ready::get(css);
        // let mut default = Ready::default();
        default.0.append(&mut self.0);
        default.0.sort_by_key(|(i, _)| *i);
        default.0.dedup();
        default.0.into_iter().collect()
    }

    fn add(&mut self, elem: (Item, Value)) { self.0.push(elem); }

    fn item_value(&self) -> &Vec<(Item, Value)> { &self.0 }
}

#[derive(Debug, Clone)]
pub struct Content(String);

impl Default for Content {
    fn default() -> Self { Self(String::from("")) }
}

#[derive(Debug, Clone, Copy)]
struct Pos(usize, usize);

impl Content {
    /// TODO: add more contents
    fn content(cssfile: CssFile) -> Self {
        match cssfile {
            CssFile::Variables => Content(String::from(str::from_utf8(VARIABLES_CSS).unwrap())),
            _ => Content::default(),
        }
    }

    /// for viewing the content
    fn get(&self) -> &str { &self.0 }

    /// for modifying the content
    fn get_mut(&mut self) -> &mut String { &mut self.0 }

    /// hypothesis: `item: value;`
    /// better to use `regex`, but for now I'm not ready :(
    fn find(&self, pat: &str) -> Result<Pos> {
        let text = self.get();
        let p1 = text.find(pat).ok_or(Error::NotFound)? + pat.len() + 2;
        let p2 = p1 + text[p1..].find(';').ok_or(Error::NotFound)?;
        dbg!(&text[p1..p2]);
        Ok(Pos(p1, p2))
    }

    /// update the content
    fn replace(&mut self, pat: &str, sub: &str) -> Result<()> {
        let Pos(p1, p2) = self.find(pat)?;
        self.get_mut().replace_range(p1..p2, sub);
        dbg!(&self.get()[p1 - 10..p2 + 5]);
        Ok(())
    }
}

#[rustfmt::skip]
macro_rules! default_item {
    ($idt:ident, $e1:expr, $e2:expr) => { (CssFile::$idt, Item($e1), Value($e2)) };
}

// TODO: add more static variables, and may remove the needless `Value` and tuples
static DEFAULT: &[(CssFile, Item, Value)] =
    &[default_item!(Pagetoc, "pagetoc", "true"),
      default_item!(Variables, "sidebar-width", "140px"),
      default_item!(Variables, "page-padding", "15px"),
      default_item!(Variables, "content-max-width", "82%"),
      default_item!(Variables, "menu-bar-height", "40px"),
      default_item!(Variables, "pagetoc-width", "13%"),
      default_item!(Variables, "pagetoc-fontsize", "14.5px"),
      default_item!(Variables, "mobile-content-max-width", "98%")];

lazy_static! {
    static ref DEFAULT_HASHMAP: HashMap<&'static Item, (&'static CssFile, &'static Value)> =
        DEFAULT.iter().map(|(css, item, value)| (item, (css, value))).collect();
}

static INDEX_PAGETOC: &str = r#"
<!-- Page table of contents -->\
<div class="sidetoc"><nav class="pagetoc"></nav></div>
"#;

#[derive(Debug, Clone)]
pub struct Theme {
    pub cssfile: CssFile,
    pub content: Content, // ultimate str to be processed
    pub ready:   Ready,   // need a func to cover the default values according to user's config
    pub pagetoc: bool,    // pagetoc releated
}

#[rustfmt::skip]
impl Default for Theme {
    fn default() -> Self {
        Self { cssfile: CssFile::Custom, content: Content::default(), 
               ready: Ready::default(), pagetoc: false, }
    }
}

impl Theme {
    /// TODO: checking user's css files is not done
    /// find pagetoc and user's css files
    fn process(mut self) -> Self {
        self.cssfile(None).ready().cotent();
        self.create_theme_file();
        self
    }

    /// TODO: `user_config` needs to search and parse css files specified by a user
    /// and the signature finally is `user_config: Option<Path>`
    fn cssfile(&mut self, user_config: Option<&'static str>) -> &mut Self {
        if let Some(_) = user_config {
            self.cssfile = CssFile::Custom;
        }
        self
    }

    fn ready(&mut self) -> &mut Self {
        self.ready = self.ready.merge(self.cssfile);
        self
    }

    #[rustfmt::skip]
    fn from(cssfile: CssFile, ready: Ready, pagetoc: bool) -> Self {
        Self { cssfile, ready, pagetoc, content: Content::default() }
    }

    fn item_value(&self, index: usize) -> Option<&(Item, Value)> { self.ready.0.get(index) }

    /// final content to be written into `theme` dir/buffer
    fn cotent(&mut self) -> &str {
        // empty content means not having processed the content
        if self.content.get() == "" {
            // TODO: need to consider a user's file
            let mut content = Content::content(self.cssfile);
            // dbg!(&text);
            // let (item, value) = self.item_value(0).unwrap();
            // dbg!(item, value);
            // content.replace(item.get(), value.get());
            for (item, value) in self.ready.item_value() {
                content.replace(item.get(), value.get());
            }
            self.content = content;
        }
        self.content.get()
    }

    /// create a css file on demand
    fn create_theme_file(&self) -> Result<()> {
        use mdbook::utils::fs::write_file;
        write_file(std::path::Path::new("theme"),
                   self.cssfile.filename(),
                   self.content.get().as_bytes()).map_err(|_| Error::FilesNotCreated)?;
        Ok(())
    }

    /// create the dirs on demand
    pub(crate) fn create_theme_dirs() -> Result<()> {
        std::fs::create_dir_all("theme/css").map_err(|_| Error::FilesNotCreated)?;
        Ok(())
    }
}
