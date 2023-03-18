use super::{Error, Result};
use std::io::Read;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Ace {
    pub theme_white: String,
    pub theme_dark: String,
    pub below_build_dir: bool,
    pub build_dir: PathBuf,   // generally `full-path/book`
    pub theme_dir: PathBuf,   // generally `theme`
    pub destination: PathBuf, // generally `full-path/book/theme-ace`
}

impl Ace {
    /// default: ace/theme/*.css in this crate;
    /// user's: ./theme/ace.css in user's book.
    /// First matched `.(*) ` is considered to be a `cssClass` .
    ///
    /// If a user both set the config in `book.toml` and have `ace-*.css` file,
    /// the config will be ignored.
    pub fn css_class_text(&self, dark: bool) -> Result<(String, String)> {
        let mut css_text = String::new();
        let ace_file = format!("ace-{}.css", if dark { "dark" } else { "white" });
        let path = self.theme_dir.join(ace_file);

        if path.exists() || self.theme_dir.join("ace.css").exists() {
            std::fs::File::open(path)
                .unwrap()
                .read_to_string(&mut css_text)
                .unwrap();
        } else if let Some(v) = self.defult_css(dark) {
            css_text = String::from(unsafe { std::str::from_utf8_unchecked(v) });
        } else {
            return Err(Error::AceNotFound);
        }

        css_text = css_text.replace(|x| x == '\n' || x == '"', " ");
        let p1 = css_text.find(".ace-").ok_or(Error::StrNotFound)?;
        let css_class =
            css_text[p1 + 1..p1 + css_text[p1..].find(' ').ok_or(Error::StrNotFound)?].to_string();
        Ok((css_class, css_text))
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
    pub fn write(&self, css_: (String, String), dark: bool) -> Result<()> {
        let file = if dark {
            "theme-tomorrow_night.js"
        } else {
            "theme-dawn.js"
        };
        let path = &self.build_dir.join("html").join(file);

        let (css_class, css_text) = css_;
        let mut content = String::new();
        std::fs::File::open(path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        content.replace_range(find(&content, "cssClass=\"")?, &css_class);
        content.replace_range(find(&content, "cssText=\"")?, &css_text);

        std::fs::write(path, content).map_err(|_| Error::FileNotWritten)?;
        Ok(())
    }

    /// organize the workflow
    pub fn run(self) -> Result<()> {
        for dark in [true, false] {
            self.write(self.css_class_text(dark)?, dark)?;
        }
        self.below_build_dir()?;
        self.remove_destination();
        Ok(())
    }

    /// move `book/html` to `book/`
    pub fn below_build_dir(&self) -> Result<()> {
        if self.below_build_dir {
            use mdbook::utils::fs::copy_files_except_ext as copy;
            let html = self.build_dir.join("html");
            copy(&html, &self.build_dir, true, None, &[]).map_err(|_| Error::DirNotCreated)?;
            std::fs::remove_dir_all(html).map_err(|_| Error::DirNotRemoved)?;
        }
        Ok(())
    }

    /// Remove `book/theme-ace`: if it's not empty, it'll not be removed.
    /// But for now, it should be empty.
    fn remove_destination(&self) {
        std::fs::remove_dir(&self.destination).unwrap_or_default();
    }
}

impl Default for Ace {
    fn default() -> Self {
        Self {
            theme_white: String::from(""),
            theme_dark: String::from(""),
            build_dir: PathBuf::from(""),
            theme_dir: PathBuf::from(""),
            destination: PathBuf::from(""),
            below_build_dir: true,
        }
    }
}

/// find the positions of double quotation marks behind cssClass or cssText
/// target: "cssClass=\"" | "cssText=\""
fn find(content: &str, target: &str) -> Result<std::ops::Range<usize>> {
    let p1 = content.find(target).ok_or(Error::StrNotFound)? + target.len();
    let p2 = p1 + content[p1..].find('"').ok_or(Error::StrNotFound)?;
    Ok(p1..p2)
}

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
