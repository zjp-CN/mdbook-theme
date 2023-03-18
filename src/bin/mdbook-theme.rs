// https://github.com/rust-lang/mdBook/blob/efb671aaf241b7f93597ac70178989a332fe85e0/examples/nop-preprocessor.rs
use clap::{Arg, ArgMatches, Command};
use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};
use mdbook_theme::PreTheme;
use semver::{Version, VersionReq};
use std::{io, process::ExitCode};

type Return = Result<(), Error>;

fn make_app() -> Command {
    let sub = Command::new("supports")
        .about("Check whether a renderer is supported by this preprocessor")
        .arg(Arg::new("renderer").required(true));
    Command::new("mdbook-theme")
        .author("zjp")
        .about(
            "A mdbook preprocessor to config theme for mdbook, \
            especially making a pagetoc on the right.",
        )
        .subcommand(sub)
}

// The return value needs to be an ExitCode due to
// [`mdbook::preprocess::CmdPreprocessor`].
fn main() -> ExitCode {
    let matches = make_app().get_matches();
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args)
    } else {
        handle_preprocessing().map_or_else(
            |err| {
                eprintln!("{err:?}");
                ExitCode::from(1)
            },
            |_| ExitCode::from(0),
        )
    }
}

// stdin
fn handle_preprocessing() -> Return {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're \
                   being called from version {}",
            PreTheme.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = PreTheme.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ExitCode {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    // whether the (html) renderer is supported
    if PreTheme.supports_renderer(renderer) {
        ExitCode::from(0)
    } else {
        ExitCode::from(1)
    }
}
