use std::convert::AsRef;
use strum_macros::AsRefStr;

#[derive(Debug, AsRefStr)]
enum CssFiles {
    S(u32),
}

fn main() {
    let a = CssFiles::S(2);
    println!("{:?}", a);
    println!("{:?}", a.as_ref());
}
