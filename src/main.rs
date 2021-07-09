use anyhow::Result;
use clap::Clap;
use dantalian::{
    dantalian::{dantalian, generate_config},
    logger::Logger,
};
use log::set_logger;
use options::{GenConfigCmd, Opts, SubCmd};
use std::collections::HashSet;

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
            let mut force: HashSet<String> = HashSet::new();
            for f in opts.force {
                force.insert(f);
            }
            for source in opts.source {
                dantalian(&source, &force).await?;
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
