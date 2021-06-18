use super::{Item, Ready, Theme, Value, DEFAULT_HASHMAP};
use crate::error::{Error, Result};
// use mdbook::theme::Theme as MdTheme;
use std::collections::HashMap;

/// TODO: `fn themes(user_config: HashMap<_, _>)`
pub fn process() {
    // user_config
    let mut input = HashMap::new();
    input.insert("sidebar-width", "200px");
    input.insert("pagetoc-width", "15%");
    // input.insert("pagetoc", "true");

    let pagetoc = input.get("pagetoc").map_or(false, |p| p.parse::<bool>().unwrap_or(false));

    let mut config = HashMap::new();

    input.into_iter()
         .map(|(item, value)| {
             // because of `Borrow<str> for &Item`, here can be `DEFAULT_HASHMAP.get(&str)`
             (*config.entry(DEFAULT_HASHMAP.get(item).unwrap().0).or_insert_with(Vec::new))
             .push((Item(item),Value(value)))
         })
         .last();

    Theme::create_theme_dirs(); // create all dirs just by once
    config.into_iter()
          .map(|(css, ready)| Theme::from(*css, Ready(ready), pagetoc).process())
          .last();
    // dbg!(&theme_config);
    // let mut theme = theme_config[0].clone();
    // theme.cotent();
    // dbg!(theme.cotent());
    // theme_config
}
