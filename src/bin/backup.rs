#![allow(unused)]
use mdbook::theme::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::str;

#[macro_use]
extern crate lazy_static;

/// TODO: consider to remove tuples
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CssFile { Variables, General, Chrome, Index, PagetocJs, PagetocCss, Custom, Pagetoc, }
// `Pagetoc` is not only single file

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
    fn borrow(&self) -> &'static str { self.0 }
}

impl Item {
    fn get(self) -> &'static str { self.0 }
}

/// by default or specified by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Value(&'static str);

impl Value {
    fn get(self) -> &'static str { self.0 }
}

#[derive(Debug, Clone)]
struct Ready(Vec<(Item, Value)>);

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

#[derive(Debug)]
enum Error {
    NotFound,
}

type Result<T> = std::result::Result<T, Error>;

fn main() {
    // user_config
    let mut input = HashMap::new();
    input.insert("sidebar-width", "200px");
    input.insert("pagetoc-width", "15%");
    // input.insert("pagetoc", "true");

    let pagetoc = input.get("pagetoc").map_or(false, |p| p.parse::<bool>().unwrap_or(false));

    let input: Vec<_> = input.into_iter().map(|(item, value)| (Item(item), Value(value))).collect();
    // dbg!("{:?}", input[0]);

    // because of `Borrow<str> for &Item`
    // dbg!(DEFAULT_HASHMAP.get("sidebar-width").unwrap());

    let mut modified_data = HashMap::new();

    for input_ in input {
        let gotten = DEFAULT_HASHMAP.get(&input_.0).unwrap();
        // dbg!(gotten.1);

        let gotten_file = gotten.0;

        let gotten_ready = (input_.0, input_.1);

        let ready = modified_data.entry(gotten_file).or_insert(Vec::new());
        ready.push(gotten_ready);
    }
    // dbg!(&modified_data);

    let theme_config: Vec<_> =
        modified_data.into_iter()
                     .map(|(css, ready)| Theme::from(*css, Ready(ready), pagetoc).preprocess())
                     .collect();
    dbg!(&theme_config);

    let mut theme = theme_config[0].clone();
    theme.cotent();
}

#[derive(Debug, Clone)]
struct Theme {
    cssfile: CssFile,
    content: Content, // ultimate str to be processed
    ready:   Ready,   // need a func to cover the default values according to user's config
    pagetoc: bool,    // pagetoc releated
}

#[rustfmt::skip]
impl Default for Theme {
    fn default() -> Self {
        Self { cssfile: CssFile::Custom, content: Content::default(), 
               ready: Ready::default(), pagetoc: false, }
    }
}

#[derive(Debug, Clone)]
struct Content(String);

impl Default for Content {
    fn default() -> Self { Self(String::from("")) }
}

#[derive(Debug, Clone, Copy)]
struct Pos(usize, usize);

impl Content {
    /// TODO: add more contens
    fn content(cssfile: CssFile) -> Self {
        match cssfile {
            CssFile::Variables => Content(String::from(str::from_utf8(VARIABLES_CSS).unwrap())),
            _ => Content::default(),
        }
    }

    fn get(&self) -> &str { &self.0 }

    fn get_mut(&mut self) -> &mut str { &mut self.0 }

    fn find(&self, pat: &str) -> Result<Pos> {
        // hypothesis: `item: value;`
        let text = self.get();
        let p1 = text.find(pat).ok_or(Error::NotFound)? + pat.len() + 2;
        let p2 = p1 + text[p1..].find(';').ok_or(Error::NotFound)?;
        dbg!(&text[p1..p2]);
        Ok(Pos(p1, p2))
    }

    fn replace(&mut self, pat: &str, sub: &str) -> Result<()> {
        let Pos(p1, p2) = self.find(pat)?;
        self.get()[p1..p2] = sub;
        Ok(())
    }
}

impl Theme {
    /// TODO: checking user's css files is not done
    /// find pagetoc and user's css files
    fn preprocess(mut self) -> Self {
        self.cssfile(None).ready();
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
    fn cotent(&mut self) {
        let content = Content::content(self.cssfile); // TODO: need to consider a user's file
                                                      // dbg!(&text);
        let (item, value) = self.item_value(0).unwrap();
        dbg!(item, value);

        dbg!(content.find(item.get()));
    }
}
