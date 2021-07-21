use std::path::PathBuf;

use clap::Clap;

/// Create and find note files for papers from a bibtex file
#[derive(Clap, Debug)]
pub struct PaperOpts {
    pub bibtex_file: PathBuf,
    #[clap(subcommand)]
    pub subcmd: PaperSubCommand,
}

#[derive(Clap, Debug)]
pub enum PaperSubCommand {
    List,
    Find
}
