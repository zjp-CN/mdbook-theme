use super::{CssFile, Item, Value};

#[rustfmt::skip]
macro_rules! default {
    ($idt:ident, $e1:expr) => { (CssFile::$idt, $e1) };
    ($idt:ident, $e1:expr, $e2:expr) => { (CssFile::$idt, Item($e1), Value($e2)) };
}

#[rustfmt::skip]
pub static CSSFILES: &[(CssFile, &'static str)] = 
    &[default!(Variables,  "css/variables.css"),
      default!(Index,      "index.hbs"),
      default!(PagetocJs,  "pagetoc.js"),
      default!(PagetocCss, "pagetoc.css"),
      default!(General,    "css/general.css"),
      default!(Chrome,     "css/chrome.css")];

#[rustfmt::skip]
pub static DEFAULT: &[(CssFile, Item, Value)] =
    &[default!(Variables, "sidebar-width",             "140px"),
      default!(Variables, "page-padding",              "15px"),
      default!(Variables, "content-max-width",         "82%"),
      default!(Variables, "menu-bar-height",           "40px"),
      default!(Variables, "pagetoc-width",             "13%"),
      default!(Variables, "pagetoc-fontsize",          "14.5px"),
      default!(Variables, "mobile-content-max-width",  "98%"),
      default!(General,   "content-padding",           "0 10px"),
      default!(General,   "content-main-margin-left",  "2%"),
      default!(General,   "content-main-margin-right", "2%"),
      default!(General,   "root-font-size",            "70%"),
      default!(General,   "body-font-size",            "105rem"),
      default!(General,   "code-font-size",            "0.9em"),
      default!(Chrome,    "sidebar-font-size",         "1em"),
      default!(Variables, "light-links-color",         "#1f1fff"),
      default!(Variables, "light-inline-code-color",   "#F42C4C")];

pub static PAGETOCCSS: &[u8] = include_bytes!("pagetoc.css");
pub static PAGETOCJS: &[u8] = include_bytes!("pagetoc.js");
