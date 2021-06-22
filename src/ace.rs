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

impl Default for Ace {
    fn default() -> Self {
        Self { theme_white:     String::from(""),
               theme_dark:      String::from(""),
               build_dir:       PathBuf::from(""), // TODO: move to `book` dir
               theme_dir:       PathBuf::from(""),
               below_build_dir: true, }
    }
}

impl Ace {
    /// default: ace/theme/*.css in this crate;
    /// user's: ./theme/ace.css in user's book.
    /// First matched `.(*) ` is considered to be a `cssClass` .
    ///
    /// If a user both set the config in `book.toml` and have `ace-*.css` file,
    /// the config will be ignored.
    pub fn text(&self, dark: bool) -> Option<String> {
        let mut css = String::new();
        let ace_file = format!("ace-{}.css", if dark { "dark" } else { "white" });
        let path = self.theme_dir.join(ace_file);
        if path.exists() || self.theme_dir.join("ace.css").exists() {
            use std::io::Read;
            std::fs::File::open(path).unwrap().read_to_string(&mut css);
        } else if let Some(bytes) = self.defult_css(dark) {
            // TODO: search static name; deal with empty theme str
            css = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
        } else {
            return None;
        }
        css.retain(|c| c != '\n');
        let p1 = css.find('.').unwrap();
        Some(format!("    t.isDark ={}0, t.cssClass = '{}',\n    t.cssText ='{}';\n",
                     if dark { '!' } else { ' ' },
                     &css[p1 + 1..p1 + css.find(' ').unwrap()],
                     css))
    }

    #[rustfmt::skip]
    fn defult_css(&self, dark: bool) -> Option<&[u8]> {
        ACE_DEFAULT.iter()
                   .find(|&(path, _)| {
                       if dark { path.ends_with(&self.theme_dark) }
                       else { path.ends_with(&self.theme_white) }
                   })
                   .map(|&(_, bytes)| bytes)
    }
}

/// TODO: placeholder:
/// `t.isDark` | `t.cssClass` | `t.cssText`
static ACE: &str = r##"
ace.define("ace/theme/tomorrow_night", 
  ["require", "exports", "module", "ace/lib/dom"], 
  function(e, t, n) {
    t.isDark = !0, t.cssClass = "ace-tomorrow-night", 
    t.cssText = '.ace-tomorrow-night .ace_gutter {background: #25282c;color: #C5C8C6}.ace-tomorrow-night .ace_print-margin {width: 1px;background: #25282c}.ace-tomorrow-night {background-color: #1D1F21;color: #C5C8C6}.ace-tomorrow-night .ace_cursor {color: #AEAFAD}.ace-tomorrow-night .ace_marker-layer .ace_selection {background: #373B41}.ace-tomorrow-night.ace_multiselect .ace_selection.ace_start {box-shadow: 0 0 3px 0px #1D1F21;}.ace-tomorrow-night .ace_marker-layer .ace_step {background: rgb(102, 82, 0)}.ace-tomorrow-night .ace_marker-layer .ace_bracket {margin: -1px 0 0 -1px;border: 1px solid #4B4E55}.ace-tomorrow-night .ace_marker-layer .ace_active-line {background: #282A2E}.ace-tomorrow-night .ace_gutter-active-line {background-color: #282A2E}.ace-tomorrow-night .ace_marker-layer .ace_selected-word {border: 1px solid #373B41}.ace-tomorrow-night .ace_invisible {color: #4B4E55}.ace-tomorrow-night .ace_keyword,.ace-tomorrow-night .ace_meta,.ace-tomorrow-night .ace_storage,.ace-tomorrow-night .ace_storage.ace_type,.ace-tomorrow-night .ace_support.ace_type {color: #B294BB}.ace-tomorrow-night .ace_keyword.ace_operator {color: #8ABEB7}.ace-tomorrow-night .ace_constant.ace_character,.ace-tomorrow-night .ace_constant.ace_language,.ace-tomorrow-night .ace_constant.ace_numeric,.ace-tomorrow-night .ace_keyword.ace_other.ace_unit,.ace-tomorrow-night .ace_support.ace_constant,.ace-tomorrow-night .ace_variable.ace_parameter {color: #DE935F}.ace-tomorrow-night .ace_constant.ace_other {color: #CED1CF}.ace-tomorrow-night .ace_invalid {color: #CED2CF;background-color: #DF5F5F}.ace-tomorrow-night .ace_invalid.ace_deprecated {color: #CED2CF;background-color: #B798BF}.ace-tomorrow-night .ace_fold {background-color: #81A2BE;border-color: #C5C8C6}.ace-tomorrow-night .ace_entity.ace_name.ace_function,.ace-tomorrow-night .ace_support.ace_function,.ace-tomorrow-night .ace_variable {color: #81A2BE}.ace-tomorrow-night .ace_support.ace_class,.ace-tomorrow-night .ace_support.ace_type {color: #F0C674}.ace-tomorrow-night .ace_heading,.ace-tomorrow-night .ace_markup.ace_heading,.ace-tomorrow-night .ace_string {color: #B5BD68}.ace-tomorrow-night .ace_entity.ace_name.ace_tag,.ace-tomorrow-night .ace_entity.ace_other.ace_attribute-name,.ace-tomorrow-night .ace_meta.ace_tag,.ace-tomorrow-night .ace_string.ace_regexp,.ace-tomorrow-night .ace_variable {color: #CC6666}.ace-tomorrow-night .ace_comment {color: #969896}.ace-tomorrow-night .ace_indent-guide {background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAACCAYAAACZgbYnAAAAEklEQVQImWNgYGBgYHB3d/8PAAOIAdULw8qMAAAAAElFTkSuQmCC) right repeat-y}';
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
