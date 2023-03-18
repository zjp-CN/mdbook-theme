use super::{default::DEFAULT, CssFile, CssFile::Invalid, Item, Ready, Theme, Value};
use std::{collections::HashMap, path::PathBuf};
use toml::{map::Map, value::Value as MdValue};

pub fn run(input: &Map<String, MdValue>, dir: PathBuf) {
    let mut input = input.to_owned();
    if input
        .remove("turn-off")
        .map_or(false, |p| p.as_bool().unwrap_or(false))
    {
        return;
    }

    Theme::create_theme_dirs(dir.clone()).unwrap_or_default(); // create all dirs just once

    if input
        .remove("pagetoc")
        .map_or(false, |p| p.as_bool().unwrap_or(false))
    {
        Theme::from(CssFile::Pagetoc, Ready::default(), dir.clone()).pagetoc(); // pagetoc defaults
    }

    let default_map: HashMap<_, _> = DEFAULT.iter().map(|(css, item, _)| (*item, *css)).collect();
    let mut config = HashMap::new(); // ultimate theme configs

    input
        .iter()
        .map(|(item, value)| {
            let item = item.as_str();
            (*config
                .entry(default_map.get(item).unwrap_or(&Invalid))
                .or_insert_with(Vec::new))
            .push((Item(item), Value(value.as_str().unwrap())))
        })
        .last();

    config
        .into_iter()
        .filter(|(css, _)| **css != Invalid) // exlude use's invalid configs
        .map(|(css, ready)| Theme::from(*css, Ready(ready), dir.clone()).process())
        .last();
}
