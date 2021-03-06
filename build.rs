use clap::IntoApp;
use clap_generate::{
    generate_to,
    generators::{Bash, Fish, Zsh},
};
include!("src/options.rs");

fn main() {
    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or_else(|| std::env::var_os("OUT_DIR"));
    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = Opts::into_app();
    app.set_bin_name("dantalian");
    generate_to::<Zsh, _, _>(&mut app, "dantalian", &outdir);
    generate_to::<Bash, _, _>(&mut app, "dantalian", &outdir);
    generate_to::<Fish, _, _>(&mut app, "dantalian", &outdir);
}
