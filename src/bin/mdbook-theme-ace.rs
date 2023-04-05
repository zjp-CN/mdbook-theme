use mdbook::renderer::RenderContext;
use mdbook_theme::{ace::Ace, theme_dir, Error, Result};

fn main() -> Result<()> {
    let ctx = RenderContext::from_json(std::io::stdin()).map_err(|_| Error::MdbookNotParsed)?;
    let mut cfg: Ace = ctx
        .config
        .get_deserialized_opt("output.theme-ace")
        .map_err(|_| Error::DeserializedFailed)?
        .ok_or(Error::DeserializedFailed)?;
    cfg.build_dir = ctx.root.join(&ctx.config.build.build_dir);
    cfg.destination = ctx.root.join(&ctx.destination);
    cfg.theme_dir = theme_dir(&ctx.root, &ctx.config);

    cfg.run()
}
