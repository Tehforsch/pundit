use clap::Clap;
use std::path::PathBuf;

/// Read and write anki databases
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Toni Peter")]
pub struct AnkiOpts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    #[clap(subcommand)]
    pub subcmd: AnkiSubCommand,
    /// The anki database to run on
    pub database_path: PathBuf,
}

#[derive(Clap, Debug)]
pub enum AnkiSubCommand {
    ListModels(ListModels),
    ListDecks(ListDecks),
}

/// List all the models (note types) in the anki database
#[derive(Clap, Debug)]
pub struct ListModels {}

/// List all the models (note types) in the anki database
#[derive(Clap, Debug)]
pub struct ListDecks {}
