use crate::error::{Error, Result};
use default::*;
use std::borrow::Borrow;
use std::borrow::Cow;
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
    fn borrow(&self) -> &str { self.0 }
}

impl<'a> Item<'a> {
    pub fn get(&self) -> &str { self.0 }
}

impl<'a> fmt::Debug for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

/// by default or specified by a user
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value<'a>(&'a str);

impl<'a> Value<'a> {
    pub fn get(&self) -> &str { self.0 }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

/// configs ready to go
#[derive(Clone)]
pub struct Ready<'a>(Vec<(Item<'a>, Value<'a>)>);

impl<'a> fmt::Display for Ready<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<'a> fmt::Debug for Ready<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0.len()) }
}

impl<'a> Default for Ready<'a> {
    fn default() -> Self { Self(vec![]) }
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
        DEFAULT.iter().filter(|(c, _, _)| *c == css).map(|(_, i, v)| (*i, *v)).collect()
    }

    pub fn item_value(&self) -> &Vec<(Item, Value)> { &self.0 }
}

#[derive(Clone)]
pub struct Content(String);

impl Default for Content {
    fn default() -> Self { Self(String::from("")) }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Content(...)") }
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
        File::open(dir.join(filename)).unwrap().read_to_string(&mut s).unwrap();
        Content(s)
    }

    /// for viewing the content
    pub fn get(&self) -> &str { &self.0 }

    /// for modifying the content
    pub fn get_mut(&mut self) -> &mut String { &mut self.0 }

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
        // eprintln!("\n{}", &self.get()[p1 - 20..p2 + 10]);
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
        // eprintln!("\n{}", &self.get()[pos - 20..pos + 20]);
        Ok(())
    }

    /// TODO: `content-max-width` + `pagetoc-width` = 95% is the best
    /// content processing in `variables.css`
    #[rustfmt::skip]
    fn variables(&mut self, item: &str, value: &str) {
        if item == "mobile-content-max-width" {
            let content = format!(
"\n@media only screen and (max-width:1439px) {{
 :root{{
    --content-max-width: {};
  }}
}}\n\n", value);
            self.insert(&content, "}", "/* Themes */");
        } else if item.starts_with("light") | item.starts_with("ayu") | 
            item.starts_with("rust") | item.starts_with("navy") | item.starts_with("coal") {
                self.fore_arg(item, value);
        } else if self.replace(item, value).is_err() {
            self.insert(&format!("\n    --{}: {};\n", item, value), ":root", "}\n").unwrap();
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
    /// 1. one word begins with/without `.` : `.content` | `body`
    /// 2. one word will very likely join more words: `.content main` | `.nav-chapters`
    fn fore_check<'a>(item: &'a str, n: usize, dot: bool, joint: &'a str) -> (String, &'a str) {
        let v: Vec<&str> = item.splitn(n, '-').collect();
        let d = if dot { "." } else { "" };
        let fore = format!("\n.{} {{", v[..n - 1].join(joint));
        (fore, v[n - 1])
    }
}

#[derive(Debug, Clone)]
pub struct Theme<'a> {
    pub cssfile: CssFile,
    pub content: Content, // ultimate str to be processed
    pub ready:   Ready<'a>,
    pub dir:     PathBuf,
}

#[rustfmt::skip]
impl<'a> Default for Theme<'a> {
    fn default() -> Self {
        Self { cssfile:  CssFile::Custom(""), content:  Content::default(),
               ready:      Ready::default(),      dir:  PathBuf::new() }
    }
}

impl<'a> Theme<'a> {
    #[rustfmt::skip]
    pub fn from(cssfile: CssFile, ready: Ready<'a>, dir:&str) -> Self {
        Self { cssfile, ready, content: Content::default(), dir: PathBuf::from(dir) }
    }

    /// TODO: Avoid rewriting when configs are not changed,
    /// or else `mdbook watch` will repeat rewriting.
    pub fn process(mut self) -> Self { self.cssfile().content().write_theme_file() }

    /// Give a default or custom virtual css file marked to help content processing.
    fn cssfile(mut self) -> Self {
        let filename = self.cssfile.filename();
        if self.dir.join(filename).exists() {
            self.cssfile = CssFile::Custom(filename);
        }
        self
    }

    /// The **ultimate** content to be written into `theme` dir.
    /// An empty content means not having processed the content.
    fn content(mut self) -> Self {
        self.content = Content::from(self.cssfile, &self.dir);
        self.content_process(None);
        self
    }

    /// process contents of different files
    #[rustfmt::skip]
    fn content_process(&mut self, filename: Option<&str>) {
        match filename.map_or_else(|| self.cssfile, |f| CssFile::variant(f)) {
            CssFile::Custom(f) => self.content_process(Some(f)),
            CssFile::Variables => self.process_variables(),
            CssFile::General   => self.process_general(),
            CssFile::Chrome    => self.process_chrome(),
            CssFile::Index     => self.process_index(),
            _ => (), // skip content processing
        }
    }

    /// create a css file on demand
    fn write_theme_file(self) -> Self {
        use std::fs::write;
        write(self.dir.join(self.cssfile.filename()), self.content.get().as_bytes()).unwrap();
        dbg!(&self);
        self
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

    /// create the dirs on demand
    pub(self) fn create_theme_dirs(dir: &str) -> Result<()> {
        std::fs::create_dir_all(PathBuf::from(dir).join("css")).map_err(|_| Error::DirNotCreated)?;
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
        let insert = r#" <!-- Page table of contents -->
                        <div class="sidetoc"><nav class="pagetoc"></nav></div>

                        "#;
        self.content.insert(insert, "<main>", "{{{ content }}}");
    }

    /// update content in `css/general.css`
    fn process_general(&mut self) {
        for (item, value) in self.ready.item_value() {
            self.content.fore_arg(item.get(), value.get());
        }
    }

    /// update content in `css/chrome.css`
    fn process_chrome(&mut self) {
        for (item, value) in self.ready.item_value() {
            // let (fore, arg) = item.get().split_once("-").unwrap();
            // self.content.fore_replace(&format!("\n.{} {{", fore), arg, value.get());
            self.content.fore_arg(item.get(), value.get());
        }
    }
}
