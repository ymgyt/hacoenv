use structopt::StructOpt;

use crate::prelude::*;

#[derive(Debug, StructOpt)]
pub struct Inspect {}

pub async fn run(opt: Inspect) {
    debug!("Start inspect {:?}", opt);
}
