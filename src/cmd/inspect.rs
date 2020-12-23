use structopt::StructOpt;

use crate::cmd::root::print_logo;
use crate::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(about = "inspect system environment")]
/// inspect command options.
pub struct Inspect {}

/// start inspect command.
pub async fn run(opt: Inspect) {
    debug!("Start inspect {:?}", opt);

    print_logo();
}
