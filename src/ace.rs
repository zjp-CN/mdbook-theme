use super::{Error, Result};
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Ace {
    pub theme_white:     String,
    pub theme_dark:      String,
    pub below_build_dir: bool,
    pub build_dir:       PathBuf,
    pub theme_dir:       PathBuf,
}

impl Ace {
    /// default: ace/theme/*.css in this crate;
    /// user's: ./theme/ace.css in user's book.
    /// First matched `.(*) ` is considered to be a `cssClass` .
    ///
    /// If a user both set the config in `book.toml` and have `ace-*.css` file,
    /// the config will be ignored.
    pub fn css_text(&self, dark: bool) -> Result<String> {
        let mut css = String::new();
        let ace_file = format!("ace-{}.css", if dark { "dark" } else { "white" });
        let path = self.theme_dir.join(ace_file);

        if path.exists() || self.theme_dir.join("ace.css").exists() {
            use std::io::Read;
            std::fs::File::open(path).unwrap().read_to_string(&mut css);
        } else if let Some(v) = self.defult_css(dark) {
            css = String::from(unsafe { std::str::from_utf8_unchecked(v) });
        } else {
            return Err(Error::AceNotFound);
        }

        css = css.replace(|x| x == '\n' || x == '\'', &" ");
        let p1 = css.find(".ace-").ok_or(Error::StrNotFound)?;
        let css_class =
            &css[p1 + 1..p1 + &css[p1..].find(' ').ok_or(Error::StrNotFound)?].to_string();
        let css_class_ = if dark { "ace-tomorrow-night" } else { "ace-dawn" };
        css = css.replace(css_class, css_class_);
        Ok(format!("    t.isDark ={}0, t.cssClass = '{}',\n    t.cssText ='{}';\n",
                   if dark { '!' } else { ' ' },
                   css_class_,
                   css))
    }

    /// get the defult css bytes matched with the user's config and a local css not found
    #[rustfmt::skip]
    pub fn defult_css(&self, dark: bool) -> Option<&[u8]> {
        ACE_DEFAULT.iter()
                   .find(|&(path, _)| {
                       if dark { path.strip_suffix(".css").unwrap().ends_with(&self.theme_dark) }
                       else { path.strip_suffix(".css").unwrap().ends_with(&self.theme_white) }
                   })
                   .map(|&(_, bytes)| bytes)
    }

    /// get the target content to be written
    pub fn write(&self, css: String, dark: bool) -> Result<()> {
        use std::io::Write;
        let file = if dark { "theme-tomorrow_night.js" } else { "theme-dawn.js" };
        // let path = &self.build_dir.join(file);
        let path = &self.build_dir.join("html").join(file);
        dbg!(&path);
        std::fs::write(path, css).map_err(|_| Error::FileNotWritten)?;
        Ok(())
    }

    /// TODO: `below_build_dir` haven't done
    pub fn run(self) -> Result<()> {
        for dark in [true, false] {
            let content = format!("{}\n{}\n{}", ACE_HEAD, self.css_text(dark)?, ACE_TAIL);
            self.write(content, dark)?;
        }
        Ok(())
    }
}

impl Default for Ace {
    fn default() -> Self {
        Self { theme_white:     String::from(""),
               theme_dark:      String::from(""),
               build_dir:       PathBuf::from(""), // TODO: move to `book` dir
               theme_dir:       PathBuf::from(""),
               below_build_dir: true, }
    }
}

static ACE_HEAD: &str = r##"
ace.define("ace/theme/tomorrow_night", 
  ["require", "exports", "module", "ace/lib/dom"], 
  function(e, t, n) {
"##;

static ACE_TAIL: &str = r##"
    var r = e("../lib/dom");
    r.importCssString(t.cssText, t.cssClass)
});

(function() {
  ace.require(["ace/theme/tomorrow_night"], function(m) {
    if (typeof module == "object" && typeof exports == "object" && module) {
        module.exports = m;
    }
  });
})();
"##;

default! {
    "./ace/theme/ambiance.css",                AMBIANCE;
    "./ace/theme/chaos.css",                   CHAOS;
    "./ace/theme/chrome.css",                  CHROME;
    "./ace/theme/clouds.css",                  CLOUDS;
    "./ace/theme/clouds_midnight.css",         CLOUDS_MIDNIGHT;
    "./ace/theme/cobalt.css",                  COBALT;
    "./ace/theme/crimson_editor.css",          CRIMSON_EDITOR;
    "./ace/theme/dawn.css",                    DAWN;
    "./ace/theme/dracula.css",                 DRACULA;
    "./ace/theme/dreamweaver.css",             DREAMWEAVER;
    "./ace/theme/eclipse.css",                 ECLIPSE;
    "./ace/theme/github.css",                  GITHUB;
    "./ace/theme/gob.css",                     GOB;
    "./ace/theme/gruvbox.css",                 GRUVBOX;
    "./ace/theme/idle_fingers.css",            IDLE_FINGERS;
    "./ace/theme/iplastic.css",                IPLASTIC;
    "./ace/theme/katzenmilch.css",             KATZENMILCH;
    "./ace/theme/kr_theme.css",                KR_THEME;
    "./ace/theme/kuroir.css",                  KUROIR;
    "./ace/theme/merbivore.css",               MERBIVORE;
    "./ace/theme/merbivore_soft.css",          MERBIVORE_SOFT;
    "./ace/theme/mono_industrial.css",         MONO_INDUSTRIAL;
    "./ace/theme/monokai.css",                 MONOKAI;
    "./ace/theme/nord_dark.css",               NORD_DARK;
    "./ace/theme/one_dark.css",                ONE_DARK;
    "./ace/theme/pastel_on_dark.css",          PASTEL_ON_DARK;
    "./ace/theme/solarized_dark.css",          SOLARIZED_DARK;
    "./ace/theme/solarized_light.css",         SOLARIZED_LIGHT;
    "./ace/theme/sqlserver.css",               SQLSERVER;
    "./ace/theme/terminal.css",                TERMINAL;
    "./ace/theme/textmate.css",                TEXTMATE;
    "./ace/theme/tomorrow.css",                TOMORROW;
    "./ace/theme/tomorrow_night_blue.css",     TOMORROW_NIGHT_BLUE;
    "./ace/theme/tomorrow_night_bright.css",   TOMORROW_NIGHT_BRIGHT;
    "./ace/theme/tomorrow_night.css",          TOMORROW_NIGHT;
    "./ace/theme/tomorrow_night_eighties.css", TOMORROW_NIGHT_EIGHTIES;
    "./ace/theme/twilight.css",                TWILIGHT;
    "./ace/theme/vibrant_ink.css",             VIBRANT_INK;
    "./ace/theme/xcode.css",                   XCODE
}
