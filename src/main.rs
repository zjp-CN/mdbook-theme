#![allow(unused)]
use mdbook::theme::*;
// use std::str;
use std::collections::HashMap;
use std::convert::AsRef;
use std::fmt;
use std::hash::Hash;
use strum_macros::AsRefStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
enum CssFiles {
    Variables(&'static [u8]),
    General(&'static [u8]),
    Chrome(&'static [u8]),
    Index(&'static [u8]),
    PagetocJs(&'static [u8]),
    PagetocCss(&'static [u8]),
}

impl fmt::Debug for CssFiles {
    /// show variant's name rather than a bunch of bytes
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CssFiles::{}", self.as_ref())
    }
}

/// from `mdboook::theme` by default or css files provided by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CssContent(&'static str);
/// 1. supported items (config args)
/// 2. item of `preprocessor.theme-pre` table in book.toml
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Item(&'static str);
/// by default or specified by a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Value(&'static str);

type Ready = (Item, Value);

// impl Debug for DefaultCss {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{} {}", print_type_of(self), stringify!(self))
//     }p
// }
//
// fn print_type_of<T>(_: &T) -> &'static str { std::any::type_name::<T>() }

macro_rules! default_item {
    ($idt:ident($e1:expr), $e2:expr, $e3:expr) => {
        (CssFiles::$idt($e1), Item($e2), Value($e3))
    };
}

static INDEX_PAGETOC: &str = r#"
<!-- Page table of contents -->\
<div class="sidetoc"><nav class="pagetoc"></nav></div>
"#;

fn main() {
    // static DEFAULT_ITEMS: &[&str] = &["a", "b"];
    // static DEFAULT_VALUES: &[&str] = &[];
    // static VARIABLES: CssFiles = CssFiles::Variables(str::from_utf8(VARIABLES_CSS).unwrap());
    static DEFAULT: &[(CssFiles, Item, Value)] =
        &[default_item!(Index(INDEX), "pagetoc", INDEX_PAGETOC),
          default_item!(Variables(VARIABLES_CSS), "sidebar-width", "140px"),
          default_item!(Variables(VARIABLES_CSS), "page-padding", "15px"),
          default_item!(Variables(VARIABLES_CSS), "content-max-width", "82%"),
          default_item!(Variables(VARIABLES_CSS), "menu-bar-height", "40px"),
          default_item!(Variables(VARIABLES_CSS), "pagetoc-width", "13%"),
          default_item!(Variables(VARIABLES_CSS), "pagetoc-fontsize", "14.5px"),
          default_item!(Variables(VARIABLES_CSS), "mobile-content-max-width", "98%")]; // TODO: add more static variables

    let items = DEFAULT.to_owned();
    // println!("{:?}", items);
    let mut input = HashMap::new();
    input.insert("sidebar-width", "200px");
    input.insert("pagetoc-width", "15%");

    let mut default_hash: HashMap<_, _> =
        DEFAULT.iter().map(|(css, item, value)| (item, (css, value))).collect();
    // println!("{:?}", default_hash);

    let input: Vec<_> = input.into_iter().map(|(item, value)| (Item(item), Value(value))).collect();
    // println!("{:?}", input[0]);

    let mut modified_data: HashMap<_, Vec<Ready>> = HashMap::new();

    for item in input {
        let gotten = default_hash.get(&item.0).unwrap();
        println!("{:?}", gotten.1);

        let gotten_file = gotten.0;

        let gotten_ready: Ready = (item.0, item.1);

        let ready = modified_data.entry(gotten_file).or_insert(Vec::new());
        ready.push(gotten_ready);

        println!("{:?}", modified_data);
    }
}
