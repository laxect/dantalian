use anyhow::Result;
use clap::Clap;
use dantalian::{
    dantalian::{dantalian, generate_config},
    logger::Logger,
};
use log::set_logger;
use options::{GenConfigCmd, Opts, SubCmd};
use std::{collections::HashSet, iter::FromIterator};

mod options;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.verbose {
        true => set_logger(Logger::init(log::LevelFilter::Trace)).unwrap(),
        false => set_logger(Logger::init(log::LevelFilter::Info)).unwrap(),
    }
    match opts.subcmd {
        None => {
            let force: HashSet<String> = HashSet::from_iter(opts.force);
            let force_all = opts.force_all;
            let is_force = |path| force_all || force.contains(&path);
            for source in opts.source {
                dantalian(&source, is_force).await?;
            }
            Ok(())
        }
        Some(subcmd) => match subcmd {
            SubCmd::GenConfig(gen_opts) => {
                let GenConfigCmd { keyword, path } = gen_opts;
                generate_config(keyword, &path).await
            }
        },
    }
}
