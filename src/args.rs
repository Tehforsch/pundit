use clap::Clap;
use std::path::PathBuf;

/// Manage notes and links between them.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Toni Peter")]
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
    Backlinks(ListBacklinks),
    Find(FindNoteInteractively),
    Verify(VerifyNotes),
    Delete(DeleteNote),
    Rename(RenameNote),
    #[cfg(feature = "anki")]
    Anki(Anki),
}

/// List notes.
#[derive(Clap, Debug)]
pub struct ListNotes {
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
}

/// List all notes that contain a link to the note
#[derive(Clap, Debug)]
pub struct ListBacklinks {
    /// The filename for which to show the backlinks
    pub filename: PathBuf,
}

/// Select a note from the list of all notes via fzf
#[derive(Clap, Debug)]
pub struct FindNoteInteractively {
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
}

/// Verify that all links are to valid files
#[derive(Clap, Debug)]
pub struct VerifyNotes {}

/// Delete a note. This will only delete the note if no other notes link to it. Otherwise it will print a list of notes linking to this note.
#[derive(Clap, Debug)]
pub struct DeleteNote {
    /// The path to the note which to delete
    pub filename: PathBuf,
}

/// Rename a note
#[derive(Clap, Debug)]
pub struct RenameNote {
    /// The path to the note which to rename
    pub filename: PathBuf,
    pub new_name: String,
}

/// Update the anki contents from the notes.
#[derive(Clap, Debug)]
#[cfg(feature = "anki")]
pub struct Anki {
    pub database: PathBuf,
}
