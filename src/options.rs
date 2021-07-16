use clap::{crate_authors, crate_description, crate_version, Clap, ValueHint};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(author=crate_authors!(), version=crate_version!(), about=crate_description!())]
pub struct Opts {
    #[clap(short, long, about = "enable verbose")]
    pub verbose: bool,
    #[clap(short, long, about = "source folders", required = false, value_hint=ValueHint::DirPath)]
    pub source: Vec<PathBuf>,
    #[clap(long, about = "dir names which you want to force re-generate", required = false)]
    pub force: Vec<String>,
    #[clap(long, about = "force re-generate all anime")]
    pub force_all: bool,
    #[clap(subcommand)]
    pub subcmd: Option<SubCmd>,
}

#[derive(Clap)]
pub enum SubCmd {
    #[clap()]
    GenConfig(GenConfigCmd),
}

#[derive(Clap)]
#[clap(about = "generate subject config")]
pub struct GenConfigCmd {
    #[clap(about = "search keyword")]
    pub keyword: Vec<String>,
    #[clap(short, long, about = "anime dir path")]
    pub path: PathBuf,
}
