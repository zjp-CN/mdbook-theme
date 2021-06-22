use mdbook_theme::ace::{Ace, ACE_DEFAULT};

fn main() {
    let s = Ace::default().text(true);
    println!("{}", s);

    // println!("{:?}", ACE_DEFAULT);
}
