use structopt::StructOpt;

use crate::config::Config;
use crate::inspect;
use crate::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(about = "fleet operations")]
pub struct Fleet {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    Init(Init),
    Inspect(Inspect),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "init fleet environment")]
pub struct Init {}

#[derive(StructOpt, Debug)]
#[structopt(about = "inspect fleet environment")]
pub struct Inspect {}

pub async fn run_init(opt: Init) {
    debug!("Start init {:?}", opt);
}

pub async fn run_inspect(opt: Inspect) {
    debug!("Start inspect {:?}", opt);

    let cfg = Config::load().unwrap();

    let inspect_result = inspect::common(&cfg.common).unwrap();

    for r in &inspect_result {
        info!("{bin:10} {state:?}",  bin = r.dep.name(), state = r.state);
    }
}
