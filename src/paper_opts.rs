use std::path::PathBuf;

use clap::Parser;

/// Create and find note files for papers from a bibtex file
#[derive(Parser, Debug)]
pub struct PaperOpts {
    pub bibtex_file: PathBuf,
    #[clap(subcommand)]
    pub subcmd: PaperSubCommand,
}

#[derive(Parser, Debug)]
pub enum PaperSubCommand {
    List,
    Find,
}
