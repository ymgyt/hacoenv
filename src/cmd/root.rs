use structopt::{clap, StructOpt};

use crate::cmd::{Fleet, Inspect};

/// initialize top level cli command from args.
pub fn initialize_from_args() -> Hacoenv {
    Hacoenv::from_args()
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "hacoenv",
    about = "hacobu local development environment setup tool",
    version(env ! ("CARGO_PKG_VERSION")),
    setting(clap::AppSettings::ArgRequiredElseHelp),
    global_settings(& [
        clap::AppSettings::ColoredHelp,
        clap::AppSettings::ColorAuto,
        clap::AppSettings::VersionlessSubcommands,
        clap::AppSettings::DisableHelpSubcommand,
        clap::AppSettings::DeriveDisplayOrder,
    ]),
)]
/// top level cli command.
pub struct Hacoenv {
    #[structopt(
        short = "v",
        long = "verbose",
        global = true,
        help = "logging verbose. (v=DEBUG, vv=TRACE)",
        parse(from_occurrences)
    )]
    /// logging verbose.
    pub verbose: u8,

    #[structopt(subcommand)]
    /// cli subcommand.
    pub subcommand: Subcommand,
}

#[derive(StructOpt, Debug)]
/// cli subcommands.
pub enum Subcommand {
    Inspect(Inspect),
    Fleet(Fleet),
}

const LOGO: &str = r#"
__    __       ___       ______   ______   .______    __    __
|  |  |  |     /   \     /      | /  __  \  |   _  \  |  |  |  |
|  |__|  |    /  ^  \   |  ,----'|  |  |  | |  |_)  | |  |  |  |
|   __   |   /  /_\  \  |  |     |  |  |  | |   _  <  |  |  |  |
|  |  |  |  /  _____  \ |  `----.|  `--'  | |  |_)  | |  `--'  |
|__|  |__| /__/     \__\ \______| \______/  |______/   \______/
"#;

pub(crate) fn print_logo() {
    if std::env::var("HACOENV_LOGO")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false)
    {
        println!("{}", LOGO);
    }
}
