use super::{Item, Ready, Theme, Value, DEFAULT_HASHMAP};
use crate::error::{Error, Result};
use mdbook::theme::Theme as MdTheme;
use std::collections::HashMap;

pub fn themes() -> Vec<Theme> {
    // user_config
    let mut input = HashMap::new();
    input.insert("sidebar-width", "200px");
    input.insert("pagetoc-width", "15%");
    // input.insert("pagetoc", "true");

    let pagetoc = input.get("pagetoc").map_or(false, |p| p.parse::<bool>().unwrap_or(false));

    let input: Vec<_> = input.into_iter().map(|(item, value)| (Item(item), Value(value))).collect();
    // dbg!("{:?}", input[0]);

    // because of `Borrow<str> for &Item`
    // dbg!(DEFAULT_HASHMAP.get("sidebar-width").unwrap());

    let mut modified_data = HashMap::new();

    for input_ in input {
        let gotten = DEFAULT_HASHMAP.get(&input_.0).unwrap();
        // dbg!(gotten.1);

        let gotten_file = gotten.0;

        let gotten_ready = (input_.0, input_.1);

        let ready = modified_data.entry(gotten_file).or_insert(Vec::new());
        ready.push(gotten_ready);
    }
    // dbg!(&modified_data);

    Theme::create_theme_dirs(); // create all dirs just by once
    let theme_config: Vec<_> =
        modified_data.into_iter()
                     .map(|(css, ready)| Theme::from(*css, Ready(ready), pagetoc).process())
                     .collect();
    // dbg!(&theme_config);

    // let mut theme = theme_config[0].clone();
    // theme.cotent();
    // dbg!(theme.cotent());
    theme_config
}
