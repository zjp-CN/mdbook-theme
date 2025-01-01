A preprocessor and a backend to config themes for [mdbook](https://github.com/rust-lang/mdBook), 
especially creating a pagetoc on the right and setting full color themes from the offical 
[ace](https://github.com/ajaxorg/ace) editor.

> Warning: this repo is not actively maintained. Variable setting from book.toml may no longer work.
> You can always modify theme files without this tool.

# installation

`mdbook-theme` package includes two CLIs :
a preprocess `mdbook-theme` and a backend `mdbook-theme-ace` .

What they are actually doing is simply replacing values within files given by user or mdbook's default.

You can get these via:

1. `cargo install`

```cmd
cargo install mdbook-theme
```

2. or build the latest from source

```cmd
git clone https://github.com/zjp-CN/mdbook-theme.git
cd mdbook-theme
cargo build
```

3. or download and unzip a [complied release](https://github.com/zjp-CN/mdbook-theme/releases),
then put them in your system path.

4. if you want to use this within github action and publish through github pages, add this in your yml file:

```yml
- name: Setup mdbook-theme latest
  env:
    GH_TOKEN: ${{ github.token }}
  run: |
    gh release download -R zjp-CN/mdbook-theme -p mdbook-theme_linux.tar.gz
    tar -xvzf mdbook-theme_linux.tar.gz
    echo $PWD >> $GITHUB_PATH
```

a full example about how to set up mdbook and mdbook-theme:
[mdbook-template](https://github.com/zjp-CN/mdbook-template) and 
[gh-pages.yml](https://github.com/zjp-CN/mdbook-template/blob/main/.github/workflows/gh-pages.yml)

Any suggestion or contribution is greatly welcomed.

# mdbook-theme

This preprocessor does a little more work to integrate [mdBook-pagetoc](https://github.com/JorelAli/mdBook-pagetoc) (pure js/css/hbs files) with [mdBook](https://github.com/rust-lang/mdBook) which currently lacks a pagetoc (to jump within titles of the opened page) .

And it makes easy modification of css just via a few lines in book.toml ( fine with or without `pagetoc` ), for example, common layout, fontsize and color settings.

* If you just want a pagetoc on the right, use this in `book.toml` :

```toml
[preprocessor.theme]
pagetoc        = true

[output.html]
additional-css = ["theme/pagetoc.css"]
additional-js  = ["theme/pagetoc.js"]
```

* If you want to config more by yourself, refer to the fully supported configs as listed below:

```toml
[preprocessor.theme]
# enable pagetoc (toc on the right)
pagetoc = true

# some variables related (defined in theme/css/variables.css)
# `content-max-width` + `pagetoc-width` = 95% seems the best
pagetoc-width = "13%"
content-max-width = "82%"
pagetoc-fontsize = "14.5px"
sidebar-width = "300px"
menu-bar-height = "40px"   # memu-bar = the bar on the top
page-padding = "15px"
mobile-content-max-width = "98%"

# layout
content-padding = "0 10px"
content-main-margin-left = "2%"
content-main-margin-right = "2%"
nav-chapters-max-width = "auto"
nav-chapters-min-width = "auto"
chapter-line-height = "2em"
section-line-height = "1.5em"

# modify some fontsizes
root-font-size = "70%"    # control the main font-size
body-font-size = "1.5rem"
code-font-size = "0.9em"
sidebar-font-size = "1em"    # sidebar = toc on the left

# modify some colors under ayu/coal/light/navy/rust theme
coal-inline-code-color = "#ffb454"
light-inline-code-color = "#F42C4C"
navy-inline-code-color = "#ffb454"
rust-inline-code-color = "#F42C4C"
light-links = "#1f1fff"
rust-links = "#1f1fff"

# if true, never read and touch the files in theme dir
turn-off = false

# If you set `pagetoc = true`, you need to set the following as well:
[output.html]
theme = "theme" # this is the default if not explicitly set
additional-css = ["theme/pagetoc.css"]  # This tool will produce it!
additional-js = ["theme/pagetoc.js"]   # This tool will produce it!
```

Attention: local files in `theme` dir are prior. So if you need or modify a theme based on the
complete default this tool provide, removing the whole `theme` dir is recommended.

# mdbook-theme-ace

This backend mainly deals with the rendered theme files that may not be handled during preprocess, specifically to modify the js/css of the [ace](https://github.com/ajaxorg/ace) editor.

```toml
# here is a must to load ace editor in mdbook
[output.html]
[output.html.playground]
editable        = true

[output.theme-ace]
theme-white     = "dawn"
theme-dark      = "tomorrow_night"
below-build-dir = true
```

full-supported official [ace theme names](https://github.com/ajaxorg/ace/tree/master/lib/ace/theme) :

```text
               ambiance | chaos          |              chrome | clouds                | clouds_midnight | cobalt    |
         crimson_editor | dawn           |             dracula | dreamweaver           |         eclipse | github    |
                    gob | gruvbox        |        idle_fingers | iplastic              |     katzenmilch | kr_theme  |
                 kuroir | merbivore      |      merbivore_soft | mono_industrial       |         monokai | nord_dark |
               one_dark | pastel_on_dark |      solarized_dark | solarized_light       |       sqlserver | terminal  |
               textmate | tomorrow       | tomorrow_night_blue | tomorrow_night_bright |  tomorrow_night |
tomorrow_night_eighties | twilight       |         vibrant_ink | xcode                 |
```

Note: for simplicity, this tool just directly modify the `cssText` in *theme-dawn.js* and *theme-tomorrow_night.js* . That is to say, if you set `theme-white = "xcode"` , you may find there is **no** *theme-xcode.css* or *theme-xcode.js* in the *build_dir* .

You are allowed to provide the `ace-dark.css` and `ace-white.css` in the `theme` dir which accords with `output.html` table to shadow the default given by the official ace. And the `theme-white/dark` configs beneath `output.theme-ace` are ignored.

Besides, for convenience, if you provide a single `ace.css` , both dark and white themes will use it! This is useful when you try the same ace config on both themes. But you're informed that `ace-dark.css` or `ace-white.css` is firstly used whenever there is `ace.css` or not . For instance, the bundle of `ace-white.css` and `ace.css` actually works as the combination of `ace-white.css` and `ace-dark.css` ; the bundle of `ace-white.css` , `ace-dark.css` and `ace.css` actually works as the combination of `ace-white.css` and `ace-dark.css` .

In short, you can download a css file form [ace theme](https://github.com/ajaxorg/ace/tree/master/src/theme) , rename it `ace.css` or `ace-dark.css` / `ace-white.css` , do minor modification about colors and put it into the `theme` dir.

`below-build-dir = true`  is the default to make output files in `html` right below `build_dir` in stead of `build_dir/html` , and there is no `build_dir/theme-post` automatically generated by mdbook. If you set `below-build-dir = false` , there will be `html` and `theme-post` dirs under *build_dir* (usually `book/`), and the `theme-post` should be empty for now.

# details about the preprocessor

<details>
  <summary>expand to see the details</summary>

## when `pagetoc = true`

```toml
[preprocessor.theme]
pagetoc = true
```

### pagetoc css

**Much appreciation for JorelAli's handy [mdBook-pagetoc](https://github.com/JorelAli/mdBook-pagetoc) !**

1. automatically add pagetoc in `index.hbs` :

```html
<!-- before in `index.hbs`                │after in `index.hbs` -->
  <div id="content" class="content">      │  <div id="content" class="content">
      <main>                              │      <main>
                                          │          <!-- Page table of contents -->
                                          │          <div class="sidetoc"><nav class="pagetoc"></nav></div>
                                          │
          {{{ content }}}                 │          {{{ content }}}
      </main>                             │      </main>
```

2. automatically  add `pagetoc.js` and `pagetoc.css` files

### `css/variables.css`

```toml
[preprocessor.theme]
pagetoc = true
# variables
pagetoc-width = "13%"
pagetoc-fontsize = "14.5px"
sidebar-width = "300px"
content-max-width = "82%"
menu-bar-height = "40px" # memu-bar = the bar on the top
page-padding = "15px"
mobile-content-max-width = "98%"
```

| `:root` variables   | default value |
| ------------------- | ------------- |
| --sidebar-width     | 300px         |
| --page-padding      | 15px          |
| --content-max-width | 750px         |
| --menu-bar-height   | 50px          |

by using `mdbook-theme`  , you can particularly specify the pagetoc width and fontsize:

| `:root` variables   | info    | set `pagetoc = true` |
| ------------------- | ------- | -------------------- |
| --sidebar-width     | default | 140px                |
| --page-padding      | default | 15px                 |
| --content-max-width | default | 82%                  |
| --menu-bar-height   | default | 40px                 |
| --pagetoc-width     | added   | 13%                  |
| --pagetoc-fontsize  | added   | 14.5px               |

Besides, this tool automatically makes content width larger if `max-width:1439px` (i.e. on the mobile device screen) when `pagetoc = true` is set in book.toml.

```css
@media only screen and (max-width:1439px) {
  :root{
    --content-max-width: 98%;
  }
}
```

### layout

```toml
[preprocessor.theme]
pagetoc = true
# layout 
content-padding = "0 10px"
content-main-margin-left = "2%"
content-main-margin-right = "2%"
nav-chapters-max-width = "auto"
nav-chapters-min-width = "auto"
chapter-line-height = "2em"
section-line-height = "1.5em"
```

```css
/* before in `css/general.css`               │ after in `css/general.css` */
  .content {                                 │    .content {
      padding: 0 15px;                       │        padding: 0 10px;
      ...                                    │        ...
  }                                          │    }
  .content main {                            │    .content main {
      margin-left: auto;                     │        margin-left: 2%;
      margin-right: auto;                    │        margin-right: 2%;
      ...                                    │        ...
  }                                          │    }
```

```css
/* before in `css/chrome.css`                │ after in `css/chrome.css` */
  .nav-chapters {                            │    .nav-chapters {
      ...                                    │        ...
      max-width: 150px;                      │        max-width: auto;
      min-width: 90px;                       │        min-width: auto;
      ...                                    │        ...
  }                                          │    }
                                             │
  .chapter {                                 │    .chapter {
      ...                                    │        ...
      line-height: 2.2em;                    │        line-height: 2.em;
  }                                          │    }
                                             │
 .section {                                  │    .section {
      ...                                    │        ...
      line-height: 1.9em;                    │        line-height: 1.5em;
  }                                          │    }
```

## set some fontsizes

To modify the fontsize in `css/general.css` :

```css
/* before in `css/general.css`               │ after in `css/general.css` */
    :root {                                  │    :root {
        font-size: 62.5%;                    │        font-size: 70%;
    }                                        │    }
                                             │
    body {                                   │    body {
        font-size: 1.6rem;                   │        font-size: 1.5rem;
      ...                                    │        ...
    }                                        │    }
                                             │
    code {                                   │    code {
        font-size: 0.875em;                  │        font-size: 0.9em;
      ...                                    │        ...
    }                                        │    }
```

and modify the fontsize of `.sidebar` in `css/chrome.css` ,

```css
/* before in `css/chrome.css`                │ after in `css/chrome.css` */
  .sidebar {                                 │    .sidebar {
      ...                                    │        ...
      font-size: 0.875em;                    │        font-size: 1em;
      ...                                    │        ...
  }                                          │    }
```

you can add `*-font-size = "value"` in book.toml:

```toml
[preprocessor.theme]
root-font-size = "70%"
body-font-size = "1.5rem"
code-font-size = "0.9em"
sidebar-font-size = "1em"
```

## set some colors

`--links` and `--inline-code-color` in `light` theme (in `css/variables.css` ) can be simply modified via this preprocessor.

These colors draws little attention to recognition for myself : )

```toml
[preprocessor.theme]
# you'are allowed to change the prefix within ayu/coal/light/navy/rust
light-links = "#1f1fff"
light-inline-code-color = "#F42C4C"
```

## if not set `pagetoc = true`

If a user did *not* set `pagetoc = true` (or equivalently `pagetoc = false`), ` Ready` will get an **empty** default, meaning this tool completely acts with user's configs.

The user can still set anything that only works with pagetoc's presence, though that setting will *not* work (and it will lie in css files).

More likely, a user may actually set `pagetoc = "true"`, then `Ready` will  get a **full** default, meaning he/she don't have to set most of the configs.

Both circumstances are **ready** to go!

</details>

## avoid repeating call on this tool when `mdbook watch`

Once `mdbook watch` detects your files changed, freshing leads to invoke `preprocessor.theme`, and `preprocessor.theme` reads from `book.toml` and `Theme` dir. When `preprocessor.theme` finds your css/js files are not consistent with what it computes, it'll cover everything concerned. (Of course, if they are consistent, no file will be rewritten.) The procedure holds back and forth, in the backyard ...

The point is that this tool, unlike the preprocessors aiming to deal with contents in md files and obliged to keep up with revision, produces theme files that are **needless to check (compute and compare) as long as nothing concerned changes**. Sadly, this tool are incapable of doing this kind of check, beacuse I haven't found a solution, causing whether to check is at the user's explicit option (see the following suggestions).

Since all theme configs are written into files under `theme` dir or appended only once, `mdbook build` will **not cause repeating** .

Therefore, if you don't modify the theme when `mdbook watch`, **run once `mdbook build`** and do **one** of the followings to avoid repeating:

1. set `turn-off = true` beneath `[preprocessor.theme]` to let this tool do nothing (such as not comparing local files with computed ones), or equivalently set the [shell env](https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html) :
   ```cmd
   export MDBOOK_preprocessor__theme__turn_off=true
   #export MDBOOK_output__html__additional_css="[]"  # if the theme dir no longer exists, don't forget to tell mdbook about it
   #export MDBOOK_output__html__additional_js="[]"   # if the theme dir no longer exists, don't forget to tell mdbook about it
   mdbook watch
   ```

   and you can restore these values:
   ```cmd
   export MDBOOK_preprocessor__theme__turn_off=false
   #export MDBOOK_output__html__additional_css='["theme/pagetoc.css"]' # if you have set this empty, don't forget to fetch it now
   #export MDBOOK_output__html__additional_js='["theme/pagetoc.js"]'   # if you have set this empty, don't forget to fetch it now
   mdbook watch
   ```

   Remember a env variable always outstrips your configs counterpart in `book.toml` , so prefer to delete the env values to restore configs:
   ```cmd
   unset MDBOOK_preprocessor__theme__turn_off MDBOOK_output__html__additional_css MDBOOK_output__html__additional_js
   ```
2. comment the table header (i.e.`#[preprocessor.theme]`) to skip this preprocessor ( `turn-off = true` actually does not prevent running this preprocessor ); if you are certain that you will never need this tool to generate files agian, it's fine to delete the whole `[preprocessor.theme] ` table and keep the `theme` dir.
3. add `theme` in your `.gitignore` file to skip `mdbook watch`'s check on `theme` dir: this is a simple but useful way if you don't mind the `theme` dir; `mdbook build` will check the `theme` dir no matter whether it's in `.gitignore` or not : )

Fisrt two suggestions also suits more-than-once `mdbook build` in order to reduce/ban computation this tool produces during preprocess.

# examples

## Rust Book

```diff
 [output.html]
-additional-css = ["ferris.css", "theme/2018-edition.css"]
-additional-js  = ["ferris.js"]
+additional-css = ["ferris.css", "theme/2018-edition.css", "theme/pagetoc.css"]
+additional-js  = ["ferris.js", "theme/pagetoc.js"]

+[preprocessor.theme]
+pagetoc                   = true
+sidebar-width             = "280px"
+content-max-width         = "75%"
+content-main-margin-left  = "5%"
+content-main-margin-right = "5%"
+root-font-size            = "80%"
+sidebar-font-size         = "0.85em"
```

before : [https://doc.rust-lang.org/book](https://doc.rust-lang.org/book)

![image](https://user-images.githubusercontent.com/25300418/230078113-72976162-667a-4986-9b49-18e309db76ad.png)

after :

![image](https://user-images.githubusercontent.com/25300418/230078346-1154052a-3869-47d6-995a-9c1a147958a3.png)

## Rust Reference

```diff
 [output.html]
-additional-css = ["theme/reference.css"]
+additional-css = ["theme/reference.css", "theme/pagetoc.css"]
+additional-js  = ["theme/pagetoc.js"]
...
+[preprocessor.theme]
+pagetoc       = true
+sidebar-width = "240px"
```

before : [https://doc.rust-lang.org/nightly/reference](https://doc.rust-lang.org/nightly/reference)

![image](https://user-images.githubusercontent.com/25300418/230078462-49634775-4743-4d6f-b425-5db693017ae6.png)

after :

![image](https://user-images.githubusercontent.com/25300418/230078568-be9ca693-95d8-4265-bb61-5af7aaa8a919.png)

## Rust by Example

**Change the code editor theme**

```diff
+[preprocessor.theme]
+pagetoc           = false
+sidebar-width     = "290px"
+content-max-width = "85%"
+root-font-size    = "75%"

+[output.html]

+[output.theme-ace]
+theme-white       = "ambiance"
+theme-dark        = "solarized_dark"
```

before : [https://doc.rust-lang.org/stable/rust-by-example](https://doc.rust-lang.org/stable/rust-by-example)

![image](https://user-images.githubusercontent.com/25300418/230078644-75016aa9-c128-4592-9bdf-ec3d54842665.png)

![image](https://user-images.githubusercontent.com/25300418/230078755-e2123299-75ee-4f79-9540-4b6ee63d28ab.png)

after :

![image](https://user-images.githubusercontent.com/25300418/230078884-f0f220d1-bf46-472a-8461-05f51a2a2def.png)

![image](https://user-images.githubusercontent.com/25300418/230079268-5bcc33be-7a50-4346-8095-4b6bfaa63e2e.png)

## Others

Rust API Guidelines (Chinese Version): [https://zjp-cn.github.io/api-guidelines](https://zjp-cn.github.io/api-guidelines)

![image](https://user-images.githubusercontent.com/25300418/230079349-cc2b503e-0b05-4b5e-a8ec-e4500d20b055.png)

The Little Book of Rust Macros (Updated & Chinese Version): [https://zjp-cn.github.io/tlborm](https://zjp-cn.github.io/tlborm)

![image](https://user-images.githubusercontent.com/25300418/230079424-0d727790-18c7-40f6-ae31-a4c8f16fa079.png)
