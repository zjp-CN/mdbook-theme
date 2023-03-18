use crate::{Error, Result};
use default::*;
use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

pub mod config;
pub mod default;

/// All cssfiles to be modified.
/// There are several aspects of configs:
/// 1. pagetoc related
/// 2. fontsize related
/// 3. color related
/// but in practice all configs are processed in unit of single file.
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CssFile {
    Variables, General, Chrome, Index, PagetocJs, PagetocCss,
    Invalid, Pagetoc, Custom(&'static str)
}

impl CssFile {
    /// get filename according to `CssFile` type
    pub fn filename(&self) -> &'static str {
        if let CssFile::Custom(filename) = self {
            filename
        } else {
            CSSFILES.iter().find(|&(css, _)| *css == *self).unwrap().1
        }
    }

    /// get `CssFile` variant according to filename
    pub fn variant(filename: &str) -> Self {
        CSSFILES.iter().find(|&(_, f)| &filename == f).unwrap().0
    }
}

/// 1. supported items (config args)
/// 2. item of `preprocessor.theme-pre` table in book.toml
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Item<'a>(&'a str);

/// useful when looking up in `HashMap<&Item, _>` just via `HashMap<&str, _>`
impl<'a> Borrow<str> for Item<'a> {
    fn borrow(&self) -> &str {
        self.0
    }
}

impl<'a> Item<'a> {
    pub fn get(&self) -> &str {
        self.0
    }
}

impl<'a> fmt::Debug for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// by default or specified by a user
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value<'a>(&'a str);

impl<'a> Value<'a> {
    pub fn get(&self) -> &str {
        self.0
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// configs ready to go
#[derive(Clone, Default)]
pub struct Ready<'a>(Vec<(Item<'a>, Value<'a>)>);

impl<'a> fmt::Display for Ready<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<'a> fmt::Debug for Ready<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.len())
    }
}

/// get `Ready` by using `iter.collect()`
impl<'a> FromIterator<(Item<'a>, Value<'a>)> for Ready<'a> {
    fn from_iter<I: IntoIterator<Item = (Item<'a>, Value<'a>)>>(iter: I) -> Self {
        let mut r = Self::default();
        for i in iter {
            r.0.push(i);
        }
        r
    }
}

/// yield default config or merge configs
impl<'a> Ready<'a> {
    /// To get a default config from a specific cssfile, which need modifying.
    /// See [`DEFAULT`] to check detailed configs.
    ///
    /// [`DEFAULT`]: ./default/static.DEFAULT.html
    #[rustfmt::skip]
    pub fn get_defualt(css: CssFile) -> Self {
        match css {
            c @ CssFile::Variables => Ready::from(c),
            c @ CssFile::General   => Ready::from(c),
            c @ CssFile::Chrome    => Ready::from(c),
            _                      => Self::default(),
        }
    }

    /// help to simplify `.get()`
    fn from(css: CssFile) -> Self {
        DEFAULT
            .iter()
            .filter(|(c, _, _)| *c == css)
            .map(|(_, i, v)| (*i, *v))
            .collect()
    }

    pub fn item_value(&self) -> &Vec<(Item, Value)> {
        &self.0
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Content(String);

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Content(...)")
    }
}

impl Content {
    /// All contents that are to modify or directly use.
    #[rustfmt::skip]
    pub fn from(cssfile: CssFile, dir: &Path) -> Self {
        use mdbook::theme::*;
        match cssfile {
            CssFile::Custom(f)  => Content::from_file(dir, f),
            CssFile::Variables  => Content::from_static(VARIABLES_CSS),
            CssFile::Index      => Content::from_static(INDEX),
            CssFile::PagetocJs  => Content::from_static(PAGETOCJS),
            CssFile::PagetocCss => Content::from_static(PAGETOCCSS),
            CssFile::Chrome     => Content::from_static(CHROME_CSS),
            CssFile::General    => Content::from_static(GENERAL_CSS),
            _                   => Content::default(),
        }
    }

    fn from_static(v: &[u8]) -> Self {
        Content(String::from(unsafe { std::str::from_utf8_unchecked(v) }))
    }

    fn from_file(dir: &Path, filename: &str) -> Self {
        use std::fs::File;
        use std::io::Read;
        let mut s = String::new();
        File::open(dir.join(filename))
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        Content(s)
    }

    /// for viewing the content
    pub fn get(&self) -> &str {
        &self.0
    }

    /// for modifying the content
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.0
    }

    /// Update the content: directly relapce a value.
    /// Useful when the item is exact or replace the first one.
    ///
    /// Hypothesis: `item: value;` .
    /// Better to use [`regex`](https://docs.rs/regex/*/regex/), but for now I'm not ready :(
    fn replace(&mut self, item: &str, value: &str) -> Result<()> {
        let text = self.get();
        let p1 = text.find(item).ok_or(Error::StrNotFound)? + item.len() + 2;
        let p2 = p1 + text[p1..].find(';').ok_or(Error::StrNotFound)?;
        self.get_mut().replace_range(p1..p2, value);
        // eprintln!("\n{}", &self.get()[p1 - 20..p2 + 10]);
        Ok(())
    }

    /// update the content with information of context:
    /// it's common to see homonymous args items in different context,
    /// so this function takes an additional foregoing hint (need two locations).
    fn fore_replace(&mut self, fore: &str, item: &str, value: &str) -> Result<()> {
        let text = self.get();
        let pfore = text.find(fore).ok_or(Error::StrNotFound)?;
        let p1 = text[pfore..].find(item).ok_or(Error::StrNotFound)? + pfore + item.len() + 2;
        let p2 = p1 + text[p1..].find(';').ok_or(Error::StrNotFound)?;
        self.get_mut().replace_range(p1..p2, value);
        Ok(())
    }

    /// Insert content, and need two str to find.
    /// The first is to find backwards;
    /// the second is to locate the inserted space right one char ahead.
    fn insert(&mut self, insert: &str, find1: &str, find2: &str) -> Result<()> {
        let text = self.get();
        let mut pos = text.find(find1).ok_or(Error::StrNotFound)?;
        pos = pos + text[pos..].find(find2).ok_or(Error::StrNotFound)? - 1;
        self.get_mut().replace_range(pos..pos + 1, insert);
        Ok(())
    }

    /// content processing in `variables.css`
    fn variables(&mut self, item: &str, value: &str) {
        if item == "mobile-content-max-width" {
            let media_on_screen = "@media only screen and (max-width:1439px)";
            if self.get().contains(media_on_screen) {
                return;
            }
            let content = format!(
                "\n{media_on_screen} {{
 :root{{
    --content-max-width: {};
  }}
}}\n\n",
                value
            );
            self.insert(&content, "}", "/* Themes */").unwrap();
        } else if item.starts_with("light")
            | item.starts_with("ayu")
            | item.starts_with("rust")
            | item.starts_with("navy")
            | item.starts_with("coal")
        {
            self.fore_arg(item, value);
        } else if self.replace(item, value).is_err() {
            self.insert(&format!("\n    --{}: {};\n", item, value), ":root", "}\n")
                .unwrap();
        }
    }

    /// deal with the config named `fore-arg: value;`
    fn fore_arg(&mut self, item: &str, value: &str) {
        for n in 2..item.split('-').count() + 1 {
            for d in [true, false] {
                for j in [" ", "-"] {
                    let (fore, arg) = Content::fore_check(item, n, d, j);
                    if self.fore_replace(&fore, arg, value).is_ok() {
                        return;
                    }
                }
            }
        }
    }

    /// parse `fore-arg`:
    /// `fore` may have multiple meaning, and it's complex:
    /// 1. one word begins with/without `.` , or even `:` : `.content` | `body` | `:root`
    /// 2. one word will very likely join more words with ` ` or `-`:
    /// `.content main` | `.nav-chapters`
    fn fore_check<'a>(item: &'a str, n: usize, dot: bool, joint: &'a str) -> (String, &'a str) {
        let v: Vec<&str> = item.splitn(n, '-').collect();
        let d = if dot { "." } else { "" };
        let fore = format!("\n{}{} {{", d, v[..n - 1].join(joint));
        (fore, v[n - 1])
    }
}

#[derive(Debug, Clone)]
pub struct Theme<'a> {
    pub cssfile: CssFile,
    pub content: Content, // ultimate str to be processed
    content_cmp: Content,
    pub ready: Ready<'a>,
    pub dir: PathBuf,
    path: PathBuf,
}

impl<'a> Default for Theme<'a> {
    fn default() -> Self {
        Self {
            cssfile: CssFile::Custom(""),
            content: Content::default(),
            ready: Ready::default(),
            dir: PathBuf::new(),
            content_cmp: Content::default(),
            path: PathBuf::new(),
        }
    }
}

impl<'a> Theme<'a> {
    #[rustfmt::skip]
    pub fn from(cssfile: CssFile, ready: Ready<'a>, dir: PathBuf) -> Self {
        Self { cssfile, ready, content: Content::default(), path: PathBuf::new(),
        dir, content_cmp: Content::default() }
    }

    /// canonical procedure
    pub fn process(self) -> Self {
        self.cssfile().content().write_theme_file()
    }

    /// Give a default or custom virtual css file marked to help content processing.
    fn cssfile(mut self) -> Self {
        let filename = self.cssfile.filename();
        self.path = self.dir.join(filename);
        if self.path.exists() {
            self.cssfile = CssFile::Custom(filename);
        }
        self
    }

    /// The **ultimate** content to be written into `theme` dir.
    /// An empty content means not having processed the content.
    fn content(mut self) -> Self {
        self.content = Content::from(self.cssfile, &self.dir);
        self.content_cmp = self.content.clone();
        self.content_process(None);
        self
    }

    /// process contents of different files
    #[rustfmt::skip]
    fn content_process(&mut self, filename: Option<&str>) {
        match filename.map_or_else(|| self.cssfile, CssFile::variant) {
            CssFile::Custom(f) => self.content_process(Some(f)),
            CssFile::Variables => self.process_variables(),
            CssFile::General   => self.process_general(),
            CssFile::Chrome    => self.process_chrome(),
            CssFile::Index     => self.process_index(),
            _ => (), // skip content processing
        }
    }

    /// Swich to another cssfile and process its content, which can repeat.
    fn ready(mut self, cssfile: CssFile) -> Self {
        self.cssfile = cssfile;
        self.ready = Ready::get_defualt(cssfile);
        self.process()
    }

    /// When `pagetoc = true` , a bunch of files need to change; if NOT true, don't call this.
    fn pagetoc(self) {
        self.ready(CssFile::Variables)
            .ready(CssFile::Index)
            .ready(CssFile::PagetocJs)
            .ready(CssFile::PagetocCss)
            .ready(CssFile::General)
            .ready(CssFile::Chrome);
    }

    /// create a css file on demand
    fn write_theme_file(self) -> Self {
        if self.content != self.content_cmp
            || ((self.cssfile == CssFile::PagetocJs || self.cssfile == CssFile::PagetocCss)
                && !self.path.exists())
        {
            std::fs::write(&self.path, self.content.get().as_bytes()).unwrap();
        }
        self
    }

    /// create the dirs on demand
    pub(self) fn create_theme_dirs(dir: PathBuf) -> Result<()> {
        std::fs::create_dir_all(dir.join("css")).map_err(|_| Error::DirNotCreated)?;
        Ok(())
    }
}

/// content processing
impl<'a> Theme<'a> {
    /// update content in `variables.css`
    fn process_variables(&mut self) {
        for (item, value) in self.ready.item_value() {
            self.content.variables(item.get(), value.get());
        }
    }

    /// update content in `index.hbs`, if and only if `pagetoc = true` for now
    fn process_index(&mut self) {
        let comment = "<!-- Page table of contents -->";
        if self.content.get().contains(comment) {
            return;
        }
        let insert = r#" {comment}
                        <div class="sidetoc"><nav class="pagetoc"></nav></div>

                        "#;
        self.content
            .insert(insert, "<main>", "{{{ content }}}")
            .unwrap();
    }

    /// update content in `css/general.css`
    fn process_general(&mut self) {
        for (item, value) in self.ready.item_value() {
            let mut item_ = item.get();
            if item_ == "root-font-size" {
                // This case is annoying:
                // field starts with `:` and value mixes with a comment
                item_ = ":root-    font-size";
            }
            self.content.fore_arg(item_, value.get());
        }
    }

    /// update content in `css/chrome.css`
    fn process_chrome(&mut self) {
        for (item, value) in self.ready.item_value() {
            self.content.fore_arg(item.get(), value.get());
        }
    }
}
