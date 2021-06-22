use super::{default::DEFAULT, CssFile, CssFile::Invalid, Item, Ready, Theme, Value};
use std::collections::HashMap;
use toml::{map::Map, value::Value as MdValue};

pub fn process(input: &Map<String, MdValue>, dir: &str) {
    let mut input = input.to_owned();
    if input.remove("turn-off").map_or(false, |p| p.as_bool().unwrap_or(false)) {
        return;
    }

    Theme::create_theme_dirs(dir).unwrap_or_default(); // create all dirs just once

    if input.remove("pagetoc").map_or(false, |p| p.as_bool().unwrap_or(false)) {
        Theme::from(CssFile::Pagetoc, Ready::default(), dir).pagetoc(); // pagetoc defaults
    }

    let default_map: HashMap<_, _> = DEFAULT.iter().map(|(css, item, _)| (*item, *css)).collect();
    let mut config = HashMap::new(); // ultimate theme configs

    input.iter()
         .map(|(item, value)| {
             let item = item.as_str();
             (*config.entry(default_map.get(item).unwrap_or(&Invalid)).or_insert_with(Vec::new))
             .push((Item(item),Value(value.as_str().unwrap())))
         })
         .last();

    config.into_iter()
          .filter(|(css, _)| **css != Invalid) // exlude use's invalid configs
          .map(|(css, ready)| Theme::from(*css, Ready(ready), dir).process())
          .last();
}
