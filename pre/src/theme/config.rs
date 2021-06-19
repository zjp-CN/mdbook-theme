use super::{CssFile, Item, Ready, Theme, Value, DEFAULT_HASHMAP};
use crate::error::{Error, Result};
use std::collections::HashMap;

/// TODO: `fn themes(user_config: HashMap<_, _>)`
pub fn process() {
    // user_config
    let mut input = HashMap::new();
    input.insert("sidebar-width", "200px");
    input.insert("pagetoc-width", "15%");
    input.insert("mobile-content-max-width", "99%");
    // `get_preprocessor` returns `Map<String, Value>`
    // so `"true"` actually is a wrapped `true`
    // input.insert("pagetoc", "true");

    Theme::create_theme_dirs(); // create all dirs just once
    let pagetoc = input.get("pagetoc").map_or(false, |p| p.parse::<bool>().unwrap_or(false));
    // set all pagetoc related defaults
    Theme::from(CssFile::Pagetoc, Ready::default(), pagetoc).pagetoc();

    let mut config = HashMap::new();

    input.into_iter()
         .map(|(item, value)| {
             // because of `Borrow<str> for &Item`, here can be `DEFAULT_HASHMAP.get(&str)`
             (*config.entry(DEFAULT_HASHMAP.get(item).unwrap().0).or_insert_with(Vec::new))
             .push((Item(item),Value(value)))
         })
         .last();

    config.into_iter()
          .map(|(css, ready)| Theme::from(*css, Ready(ready), pagetoc).process())
          .last();
}
