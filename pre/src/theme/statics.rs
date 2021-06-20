use super::{CssFile, Item, Value};

#[rustfmt::skip]
macro_rules! default {
    ($idt:ident, $e1:expr) => { (CssFile::$idt, $e1) };
    ($idt:ident, $e1:expr, $e2:expr) => { (CssFile::$idt, Item($e1), Value($e2)) };
}

// TODO: add more
#[rustfmt::skip]
pub static CSSFILES: &[(CssFile, &'static str)] = 
    &[default!(Variables,  "css/variables.css"),
      default!(Index,      "index.hbs"),
      default!(PagetocJs,  "pagetoc.js"),
      default!(PagetocCss, "pagetoc.css"),
    ];

// TODO: add more static variables, and may remove the needless `Value` and tuples
pub static DEFAULT: &[(CssFile, Item, Value)] =
    &[default!(Variables, "sidebar-width", "140px"),
      default!(Variables, "page-padding", "15px"),
      default!(Variables, "content-max-width", "82%"),
      default!(Variables, "menu-bar-height", "40px"),
      default!(Variables, "pagetoc-width", "13%"),
      default!(Variables, "pagetoc-fontsize", "14.5px"),
      default!(Variables, "mobile-content-max-width", "98%")];

pub static PAGETOCCSS: &[u8] = include_bytes!("pagetoc.css");
pub static PAGETOCJS: &[u8] = include_bytes!("pagetoc.js");
