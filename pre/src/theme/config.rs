use super::{CssFile, CssFile::Invalid, Item, Ready, Theme, Value, DEFAULT};
use crate::error::{Error, Result};
use std::collections::HashMap;
use toml::map::Map;
use toml::value;

/// TODO: `fn themes(user_config: HashMap<_, _>)`
pub fn process<'a>(input: &'a Map<String, value::Value>) {
    // user_config
    // let mut input = HashMap::new();
    // input.insert("sidebar-width", "200px");
    // input.insert("pagetoc-width", "15%");
    // input.insert("mobile-content-max-width", "99%");
    // `get_preprocessor` returns `Map<String, Value>`
    // so `"true"` actually is a wrapped `true`
    // input.insert("pagetoc", "true");

    Theme::create_theme_dirs(); // create all dirs just once

    let mut input = input.to_owned();
    if input.remove("pagetoc") // set all pagetoc related defaults
            .map_or(false, |p| p.as_bool().unwrap_or(false))
    {
        Theme::from(CssFile::Pagetoc, Ready::default()).pagetoc(); // process pagetoc defaults
    }

    let default_map: HashMap<Item<'a>, CssFile> =
        DEFAULT.iter().map(|(css, item, _)| (*item, *css)).collect();
    let mut config = HashMap::new(); // ultimate theme configs

    dbg!(&input);

    input.iter()
         .map(|(item, value)| {
             let item = item.as_str();
             (*config.entry(default_map.get(item).unwrap_or(&Invalid)).or_insert_with(Vec::new))
             .push((Item(item),Value(value.as_str().unwrap())))
         })
         .last();

    config.into_iter()
          .filter(|(css, _)| **css != Invalid) // exlude use's invalid configs
          .map(|(css, ready)| Theme::from(*css, Ready(ready)).process())
          .last();
}
