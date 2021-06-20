use super::{statics::DEFAULT, CssFile, CssFile::Invalid, Item, Ready, Theme, Value};
use crate::error::{Error, Result};
use std::collections::HashMap;
use toml::{map::Map, value::Value as MdValue};

/// TODO: `fn themes(user_config: HashMap<_, _>)`
pub fn process(input: &Map<String, MdValue>) {
    // let mut user_config = mdbook::config::Config::from_disk("book.toml").unwrap();
    // let input = user_config.get_mut("preprocessor.theme-pre").unwrap().as_table_mut().unwrap();

    Theme::create_theme_dirs(); // create all dirs just once

    let mut input = input.to_owned();
    if input.remove("pagetoc").map_or(false, |p| p.as_bool().unwrap_or(false)) {
        Theme::from(CssFile::Pagetoc, Ready::default()).pagetoc(); // process pagetoc defaults
    }

    let default_map: HashMap<_, _> = DEFAULT.iter().map(|(css, item, _)| (*item, *css)).collect();
    let mut config = HashMap::new(); // ultimate theme configs

    // dbg!(&input);

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

// TODO: check user's `HtmlConfig.theme_dir` in case of not the default `./theme` dir
// User's book.toml can't be modified, so user must set the following on his own:
// [output.html]
// additional-css = ["theme/pagetoc.css"]
// additional-js = ["theme/pagetoc.js"]
// fn additional(cfg: &mut mdbook::config::Config) {
//     cfg.set("output.html.additional-css",
//             MdValue::Array(vec![MdValue::String("pagetoc.css".to_string())]));
//     cfg.set("output.html.additional-js",
//             MdValue::Array(vec![MdValue::String("pagetoc.js".to_string())]));
//     dbg!(cfg);
// }
