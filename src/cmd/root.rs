use structopt::{clap, StructOpt};

use crate::cmd::Inspect;

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
pub struct Hacoenv {
    #[structopt(
        short = "v",
        long = "verbose",
        global = true,
        help = "logging verbose. (v=DEBUG, vv=TRACE)",
        parse(from_occurrences)
    )]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    Inspect(Inspect),
}
