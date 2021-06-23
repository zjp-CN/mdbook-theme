use super::{CssFile, Item, Value};

#[rustfmt::skip]
pub static CSSFILES: &[(CssFile, &str)] = 
    &[default!(Variables,  "css/variables.css"),
      default!(Index,      "index.hbs"),
      default!(PagetocJs,  "pagetoc.js"),
      default!(PagetocCss, "pagetoc.css"),
      default!(General,    "css/general.css"),
      default!(Chrome,     "css/chrome.css")];

#[rustfmt::skip]
pub static DEFAULT: &[(CssFile, Item, Value)] =
    &[/*                   pagetoc related                      */
      default!(Variables, "sidebar-width",             "140px"),
      default!(Variables, "page-padding",              "15px"),
      default!(Variables, "content-max-width",         "82%"),
      default!(Variables, "menu-bar-height",           "40px"),
      default!(Variables, "pagetoc-width",             "13%"),
      default!(Variables, "pagetoc-fontsize",          "14.5px"),
      default!(Variables, "mobile-content-max-width",  "98%"),
      default!(General,   "content-padding",           "0 10px"),
      default!(General,   "content-main-margin-left",  "2%"),
      default!(General,   "content-main-margin-right", "2%"),
      default!(Chrome,    "nav-chapters-max-width",    "auto"),
      default!(Chrome,    "nav-chapters-min-width",    "auto"),
      default!(Chrome,    "chapter-line-height",       "2em"),
      default!(Chrome,    "section-line-height",       "1.5em"),
      /*                   font-size related                    */
      default!(General,   "root-font-size",            "70%"),
      default!(General,   "body-font-size",            "1.5rem"),
      default!(General,   "code-font-size",            "0.9em"),
      default!(Chrome,    "sidebar-font-size",         "1em"),
      /*                   color related                        */
      default!(Variables, "light-links",               "#1f1fff"),
      default!(Variables, "light-inline-code-color",   "#F42C4C"),
      default!(Variables, "rust-links",                "#2b79a2"),
      default!(Variables, "rust-inline-code-color",    "#6e6b5e"),
      default!(Variables, "navy-links",                "#2b79a2"),
      default!(Variables, "navy-inline-code-color",    "#c5c8c6"),
      default!(Variables, "coal-links",                "#2b79a2"),
      default!(Variables, "coal-inline-code-color",    "#c5c8c6"),
      default!(Variables, "ayu-links",                 "#0096cf"),
      default!(Variables, "ayu-inline-code-color",     "#ffb454")];

pub static PAGETOCCSS: &[u8] = include_bytes!("pagetoc.css");
pub static PAGETOCJS: &[u8] = include_bytes!("pagetoc.js");
