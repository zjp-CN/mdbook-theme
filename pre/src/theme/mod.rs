use mdbook::theme::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::str;

use crate::error::{Error, Result};

pub mod config;

/// All cssfiles to be modified.
/// `Pagetoc` is not only single file
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssFile { Variables, General, Chrome, Index, PagetocJs, PagetocCss, Custom, Pagetoc, }

impl CssFile {
    pub fn filename(&self) -> &str {
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
pub struct Item(&'static str);

/// useful when looking up in `HashMap<&Item, _>`
impl Borrow<str> for &Item {
    fn borrow(&self) -> &str { self.0 }
}

impl Item {
    pub fn get(&self) -> &str { self.0 }
}

/// by default or specified by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(&'static str);

impl Value {
    pub fn get(&self) -> &str { self.0 }
}

/// configs ready to go
#[derive(Debug, Clone)]
pub struct Ready(Vec<(Item, Value)>);

impl Default for Ready {
    fn default() -> Self { Self(vec![]) }
}

/// get `Ready` by using `iter.collect()`
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
        DEFAULT.iter().filter(|(c, _, _)| *c == css).map(|(_, i, v)| (*i, *v)).collect()
    }

    fn add(&mut self, elem: (Item, Value)) { self.0.push(elem); }

    pub fn item_value(&self) -> &Vec<(Item, Value)> { &self.0 }
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
    pub fn from(cssfile: CssFile) -> Self {
        match cssfile {
            CssFile::Variables => Content(String::from(str::from_utf8(VARIABLES_CSS).unwrap())),
            _ => Content::default(),
        }
    }

    /// for viewing the content
    pub fn get(&self) -> &str { &self.0 }

    /// for modifying the content
    pub fn get_mut(&mut self) -> &mut String { &mut self.0 }

    /// hypothesis: `item: value;`
    /// better to use `regex`, but for now I'm not ready :(
    fn find(&self, pat: &str) -> Result<Pos> {
        let text = self.get();
        let p1 = text.find(pat).ok_or(Error::StrNotFound)? + pat.len() + 2;
        let p2 = p1 + text[p1..].find(';').ok_or(Error::StrNotFound)?;
        dbg!(&text[p1..p2]);
        Ok(Pos(p1, p2))
    }

    /// update the content
    fn replace(&mut self, pat: &str, sub: &str) -> Result<()> {
        let Pos(p1, p2) = self.find(pat)?;
        self.get_mut().replace_range(p1..p2, sub);
        dbg!(&self.get()[p1 - 20..p2 + 5]);
        Ok(())
    }

    /// Insert content, and need two str to find.
    /// The first is to find backwards;
    /// the second is to locate the inserted space right one char ahead.
    fn insert(&mut self, insert: &str, find1: &str, find2: &str) -> Result<()> {
        let text = self.get();
        let mut pos = text.find(find1).ok_or(Error::StrNotFound)?;
        pos = pos + text[pos..].find(find2).ok_or(Error::StrNotFound)? - 1;
        self.get_mut().replace_range(pos..pos + 1, &insert);
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
        } else {
            if self.replace(pat, sub).is_err() {
                self.insert(&format!("\n    --{}: {};\n", pat, sub), ":root", "}\n");
            }
        }
    }
}

#[rustfmt::skip]
macro_rules! default {
    ($idt:ident, $e1:expr, $e2:expr) => { (CssFile::$idt, Item($e1), Value($e2)) };
}

// TODO: add more static variables, and may remove the needless `Value` and tuples
pub static DEFAULT: &[(CssFile, Item, Value)] =
    &[default!(Pagetoc, "pagetoc", "true"),
      default!(Variables, "sidebar-width", "140px"),
      default!(Variables, "page-padding", "15px"),
      default!(Variables, "content-max-width", "82%"),
      default!(Variables, "menu-bar-height", "40px"),
      default!(Variables, "pagetoc-width", "13%"),
      default!(Variables, "pagetoc-fontsize", "14.5px"),
      default!(Variables, "mobile-content-max-width", "98%")];

lazy_static! {
    /// TODO: remove `value`
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
    pub fn process(mut self) {
        self.cssfile(None).ready().cotent();
        self.create_theme_file();
    }

    /// TODO: `user_config` needs to search and parse css files specified by a user
    /// and the signature finally is `user_config: Option<Path>`
    fn cssfile(&mut self, user_config: Option<&'static str>) -> &mut Self {
        if user_config.is_some() {
            self.cssfile = CssFile::Custom;
        }
        self
    }

    /// merge user's config
    ///
    /// If a user did not set `pagetoc = "true"` , `Ready` will get en **empty** default.
    /// But he can still set something only working with pagetoc's presence,
    /// even though that setting will *not* work (and it will lie in css files).
    ///
    /// More likely, a user may actually set `pagetoc = "true"` ,
    /// then he'll get a **full** default.
    ///
    /// Both circumstances are **ready** to go!
    fn ready(&mut self) -> &mut Self {
        let mut default = if self.pagetoc { Ready::get(self.cssfile) } else { Ready::default() };
        default.0.append(&mut self.ready.0);
        default.0.reverse();
        default.0.sort_by_key(|(i, _)| *i);
        default.0.dedup_by(|(ref a, _), (ref b, _)| b.eq(a));
        self.ready = default.0.into_iter().collect();
        self
    }

    #[rustfmt::skip]
    pub fn from(cssfile: CssFile, ready: Ready, pagetoc: bool) -> Self {
        Self { cssfile, ready, pagetoc, content: Content::default() }
    }

    // pub fn item_value(&self, index: usize) -> Option<&(Item, Value)> { self.ready.0.get(index) }

    /// create a css file on demand
    fn create_theme_file(&self) -> Result<()> {
        use mdbook::utils::fs::write_file;
        write_file(std::path::Path::new("theme"),
                   self.cssfile.filename(),
                   self.content.get().as_bytes()).map_err(|_| Error::FileNotCreated)?;
        Ok(())
    }

    /// create the dirs on demand
    pub(self) fn create_theme_dirs() -> Result<()> {
        std::fs::create_dir_all("theme/css").map_err(|_| Error::DirNotCreated)?;
        Ok(())
    }

    /// final content to be written into `theme` dir
    fn cotent(&mut self) -> &str {
        dbg!(&self);
        // empty content means not having processed the content
        if self.content.get() == "" {
            // TODO: need to consider a user's file
            self.content = Content::from(self.cssfile);
            self.content_process();
        }
        self.content.get()
    }

    /// process contents of different files
    fn content_process(&mut self) {
        match self.cssfile {
            CssFile::Variables => self.process_variables(),
            // TODO: add more branches
            _ => (),
        }
    }

    /// update content in `variables.css`
    fn process_variables(&mut self) {
        for (item, value) in self.ready.item_value() {
            self.content.variables(item.get(), value.get());
        }
    }

    /// When `pagetoc = "true"`, a bunch of files need to change;
    /// when NOT true, do nothing.
    fn pagetoc(self) {
        if self.pagetoc && self.cssfile == CssFile::Pagetoc {
            // TODO
            Theme::from(CssFile::Variables, Ready::get(CssFile::Variables), true);
        }
    }
}
