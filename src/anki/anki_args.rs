use std::path::PathBuf;

use clap::Parser;

/// Read and write anki databases
#[derive(Parser)]
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

#[derive(Parser, Debug)]
pub enum AnkiSubCommand {
    ListModels(ListModels),
    ListDecks(ListDecks),
    ListFields(ListFields),
    ListTemplates(ListTemplates),
}

/// List all the models (note types) in the anki database
#[derive(Parser, Debug)]
pub struct ListModels {}

/// List all the models (note types) in the anki database
#[derive(Parser, Debug)]
pub struct ListDecks {}

/// List all the fields for a given model (note type)
#[derive(Parser, Debug)]
pub struct ListFields {
    pub model: String,
}

/// List all the card templates for a given model (note type)
#[derive(Parser, Debug)]
pub struct ListTemplates {
    pub model: String,
}
