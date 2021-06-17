#![allow(unused)]
#![feature(const_str_from_utf8_unchecked)]
use mdbook::theme::*;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::str;
use strum_macros::AsRefStr;

#[macro_use]
extern crate lazy_static;

/// TODO: consider to remove tuples
#[derive(Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
enum CssFile {
    Variables(&'static [u8]),
    General(&'static [u8]),
    Chrome(&'static [u8]),
    Index(&'static [u8]),
    PagetocJs(&'static [u8]),
    PagetocCss(&'static [u8]),
    Custom(&'static str),
    Pagetoc, // not only single file
}

impl fmt::Debug for CssFile {
    /// show variant's name rather than a bunch of bytes
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CssFiles::{}", <CssFile as AsRef<str>>::as_ref(self))
    }
}

/// TODO: may not need this (use `match` instead)
impl Borrow<[u8]> for CssFile {
    fn borrow(&self) -> &'static [u8] {
        match self {
            CssFile::Pagetoc => &[],
            CssFile::Custom(x) => &[],
            CssFile::Variables(x) => x,
            CssFile::General(x) => x,
            CssFile::Chrome(x) => x,
            CssFile::Index(x) => x,
            CssFile::PagetocJs(x) => x,
            CssFile::PagetocCss(x) => x,
        }
    }
}

/// 1. supported items (config args)
/// 2. item of `preprocessor.theme-pre` table in book.toml
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Item(&'static str);

// impl Ord for Item {
//     fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
// }

/// useful when looking up in `HashMap<&Item, _>`
impl Borrow<str> for &Item {
    fn borrow(&self) -> &'static str { self.0 }
}

/// by default or specified by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Value(&'static str);

#[derive(Debug, Clone)]
struct Ready(Vec<(Item, Value)>);

impl Default for Ready {
    fn default() -> Self { Self(vec![]) }
}

static VARIABLES: &[(&str, &str)] = &[("sidebar-width", "140px"),
                                      ("page-padding", "15px"),
                                      ("content-max-width", "82%"),
                                      ("menu-bar-height", "40px"),
                                      ("pagetoc-width", "13%"),
                                      ("pagetoc-fontsize", "14.5px"),
                                      ("mobile-content-max-width", "98%")];

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
    fn from(css: CssFile) -> Self {
        match css {
            // CssFiles::Variables(_) => Self(vec![item_value!("sidebar-width", "140px"),
            //                                     item_value!("page-padding", "15px"),
            //                                     item_value!("content-max-width", "82%"),
            //                                     item_value!("menu-bar-height", "40px"),
            //                                     item_value!("pagetoc-width", "13%"),
            //                                     item_value!("pagetoc-fontsize", "14.5px"),
            //                                     item_value!("mobile-content-max-width", "98%")]),
            CssFile::Variables(_) => {
                VARIABLES.into_iter().map(|(i, v)| (Item(i), Value(v))).collect()
            }
            _ => Self::default(),
        }
    }

    /// `user_config` can be derive from `iter::collect`
    fn merge(&mut self) -> Self {
        let mut default = Ready::default();
        default.0.append(&mut self.0);
        default.0.sort_by_key(|(i, v)| *i);
        default.0.dedup();
        default.0.into_iter().collect()
    }

    // fn new() -> Self { Self::default() }

    fn add(&mut self, elem: (Item, Value)) { self.0.push(elem); }

    // fn pagetoc(&self) -> bool { self.0.contains(&(Item("pagetoc"), Value("true"))) }
}

macro_rules! default_item {
    (Pagetoc,"pagetoc","true") => {
        (CssFile::Pagetoc, Item("pagetoc"), Value("true"))
    };
    ($idt:ident($e1:expr), $e2:expr, $e3:expr) => {
        (CssFile::$idt($e1), Item($e2), Value($e3))
    };
}

// TODO: add more static variables, and may remove the needless `Value` and tuples
static DEFAULT: &[(CssFile, Item, Value)] =
    &[default_item!(Pagetoc, "pagetoc", "true"),
      default_item!(Variables(VARIABLES_CSS), "sidebar-width", "140px"),
      default_item!(Variables(VARIABLES_CSS), "page-padding", "15px"),
      default_item!(Variables(VARIABLES_CSS), "content-max-width", "82%"),
      default_item!(Variables(VARIABLES_CSS), "menu-bar-height", "40px"),
      default_item!(Variables(VARIABLES_CSS), "pagetoc-width", "13%"),
      default_item!(Variables(VARIABLES_CSS), "pagetoc-fontsize", "14.5px"),
      default_item!(Variables(VARIABLES_CSS), "mobile-content-max-width", "98%")];

lazy_static! {
    static ref DEFAULT_HASHMAP: HashMap<&'static Item, (&'static CssFile, &'static Value)> =
        DEFAULT.iter().map(|(css, item, value)| (item, (css, value))).collect();
}

static INDEX_PAGETOC: &str = r#"
<!-- Page table of contents -->\
<div class="sidetoc"><nav class="pagetoc"></nav></div>
"#;

// TODO: after removing `CssFile` tuples, add more static &str
// static CONTENT_VARIABLES: &str = unsafe { str::from_utf8_unchecked(VARIABLES_CSS) };

fn main() {
    // user_config
    let mut input = HashMap::new();
    input.insert("sidebar-width", "200px");
    input.insert("pagetoc-width", "15%");
    input.insert("pagetoc", "true");

    let pagetoc = input.get("pagetoc").map_or(false, |p| p.parse::<bool>().unwrap_or(false));

    let input: Vec<_> = input.into_iter().map(|(item, value)| (Item(item), Value(value))).collect();
    // dbg!("{:?}", input[0]);

    // because of `Borrow<str> for &Item`
    // dbg!(DEFAULT_HASHMAP.get("sidebar-width").unwrap());

    let mut modified_data = HashMap::new();

    for item in input {
        let gotten = DEFAULT_HASHMAP.get(&item.0).unwrap();
        // dbg!(gotten.1);

        let gotten_file = gotten.0;

        let gotten_ready = (item.0, item.1);

        let ready = modified_data.entry(gotten_file).or_insert(Vec::new());
        ready.push(gotten_ready);
    }
    // dbg!(&modified_data);

    let theme_config: Vec<_> = modified_data.into_iter()
                                            .map(|(css, ready)| {
                                                Theme { cssfile: *css,
                                                        ready:   Ready(ready),
                                                        pagetoc: pagetoc, }.preprocess()
                                            })
                                            .collect();
    dbg!(&theme_config);
}

#[derive(Debug, Clone)]
struct Theme {
    cssfile: CssFile,
    // content: Content, // need a func to retrieve from mdbook or user
    ready:   Ready, // need a func to cover the default values according to user's config
    pagetoc: bool,  // pagetoc releated
}

impl Default for Theme {
    fn default() -> Self {
        Self { cssfile: CssFile::Custom(""),
               // content: Content(""),
               ready:   Ready::default(),
               pagetoc: false, }
    }
}

impl Theme {
    // fn pagetoc(&mut self) -> &mut Self {
    //     self.pagetoc = self.ready.pagetoc();
    //     self
    // }

    /// TODO: `user_config` needs to search and parse css files specified by a user
    /// and the signature finally is `user_config: Option<Path>`
    fn cssfile(&mut self, user_config: Option<&'static str>) -> &mut Self {
        if let Some(css) = user_config {
            self.cssfile = CssFile::Custom(css);
        }
        self
    }

    fn ready(&mut self) -> &mut Self {
        self.ready = self.ready.merge();
        self
    }

    /// TODO: checking user's css files is not done
    /// find pagetoc and user's css files
    fn preprocess(mut self) -> Self {
        self.cssfile(None).ready();
        self
    }

    // final content to be written into `theme` dir
    // fn cotent(&'static mut self) {
    //     match self.cssfile {
    //         CssFile::Custom(x) => {
    //             self.content = Content(x);
    //         }
    //         _ => {
    //             self.content = Content(str::from_utf8(self.cssfile.borrow()).unwrap());
    //         }
    //     }
    // }
}
