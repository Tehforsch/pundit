use clap::Clap;
use std::path::PathBuf;

/// Manage notes and links between them.
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Toni Peter")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    /// The note folder to run on
    pub folder: Option<PathBuf>,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    List(ListNotes),
    Backlinks(Backlinks),
    Find(FindNoteInteractively),
}

/// List notes.
#[derive(Clap, Debug)]
pub struct ListNotes {
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
}

/// A subcommand for controlling testing
#[derive(Clap, Debug)]
pub struct Backlinks {
    /// The filename for which to show the backlinks
    pub filename: PathBuf,
}

/// A subcommand for controlling testing
#[derive(Clap, Debug)]
pub struct FindNoteInteractively {
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
}
