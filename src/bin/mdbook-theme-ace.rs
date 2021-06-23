use mdbook::renderer::RenderContext;
use mdbook_theme::{ace::Ace, Error, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    let ctx = RenderContext::from_json(std::io::stdin()).map_err(|_| Error::MdbookNotParsed)?;
    let mut cfg: Ace = ctx.config
                          .get_deserialized_opt("output.theme-ace")
                          .map_err(|_| Error::DeserializedFailed)?
                          .ok_or(Error::DeserializedFailed)?;
    cfg.build_dir = ctx.root.join(&ctx.config.build.build_dir);
    cfg.destination = ctx.root.join(&ctx.destination);
    cfg.theme_dir = ctx.config
                       .get("output.html.theme")
                       .map_or(PathBuf::from("theme"), |t| PathBuf::from(t.as_str().unwrap()));

    cfg.run()
}
