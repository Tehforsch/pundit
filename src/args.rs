use clap::Clap;
use std::{path::PathBuf, str::FromStr};

use crate::{filter_options::FilterOptions, journal_opts::JournalOpts};

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
    pub folder: PathBuf,
    /// The path to the database in which to store the notes and their links for fast access times
    pub database: Option<PathBuf>,
    /// Run only on the top level folder.
    #[clap(short, long)]
    pub singledir: bool,
    /// Add identifying lines to the beginning and the end of stdout so that output can be more easily parsed
    /// from terminal output in emacs (via term-char-mode)
    #[clap(short, long)]
    pub add_identifier:bool
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    List(ListNotes),
    Link(GetLinkTextInteractively),
    ShowLink(GetLinkText),
    ListBacklinks(ListBacklinks),
    Backlinks(FindBacklinks),
    Find(FindNoteInteractively),
    New(NewNote),
    Delete(DeleteNote),
    Rename(RenameNote),
    Pankit(Pankit),
    PankitGetNote(PankitGetNote),
    ListGraph(ListGraph),
    Graph(FindGraph),
    Journal(JournalOpts),
}

/// List notes.
#[derive(Clap, Debug)]
pub struct ListNotes {
    #[clap(subcommand)]
    pub filter: Option<FilterOptions>,
}

/// Interactively choose note 2 and then display a (relative) link from note 1 to note 2.
#[derive(Clap, Debug)]
pub struct GetLinkTextInteractively {
    pub note1: PathBuf,
    #[clap(subcommand)]
    pub filter: Option<FilterOptions>,
}

/// Display a (relative) link from note 1 to note 2.
#[derive(Clap, Debug)]
pub struct GetLinkText {
    /// The filename in which the file is going to be written to
    pub note1: PathBuf,
    /// The filename to link to
    pub note2: PathBuf,
}

/// List all notes that contain a link to the note
#[derive(Clap, Debug)]
pub struct ListBacklinks {
    /// The filename for which to show the backlinks
    pub filename: PathBuf,
}

#[derive(Clap, Debug)]
/// Interactively select a note out of all notes that contain a link to the note
pub struct FindBacklinks {
    /// The filename for which to show the backlinks
    pub filename: PathBuf,
}

/// Select a note from the list of all notes via fzf
#[derive(Clap, Debug)]
pub struct FindNoteInteractively {
    #[clap(subcommand)]
    pub filter: Option<FilterOptions>,
}

/// Create a new note with a given title (first ensure that it does not exist already).
#[derive(Clap, Debug)]
pub struct NewNote {
    /// Optional: Only list notes which contain this string in the title
    pub title: String,
}

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

/// Select a note interactively from the graph component for a specific note
#[derive(Clap, Debug)]
pub struct FindGraph {
    /// The path of the note
    pub filename: PathBuf,
}

/// List all notes in the graph component for a specific note
#[derive(Clap, Debug)]
pub struct ListGraph {
    /// The path of the note
    pub filename: PathBuf,
}

/// Update the anki contents from the notes.
#[derive(Clap, Debug)]
pub struct Pankit {
    /// The path of the anki database to update
    pub database: PathBuf,
    /// The path of the pankit database which is used for synchronization of the pundit notes and anki database
    pub pankit_db: PathBuf,
    /// How to deal with conflicting contents between anki and pundit that cannot be resolved automatically
    // #[clap(subcommand)]
    #[clap(default_value = "error")]
    #[clap(possible_values = &["ignore", "error", "pundit", "anki"], default_value = "error")]
    pub conflict_handling: ConflictHandling,
}

impl FromStr for ConflictHandling {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Ok(ConflictHandling::GiveError),
            "ignore" => Ok(ConflictHandling::Ignore),
            "pundit" => Ok(ConflictHandling::Pundit),
            "anki" => Ok(ConflictHandling::Anki),
            _ => Err("no match"),
        }
    }
}

/// Add a pankit note by generating an id, allowing to interactively select model/deck and adding empty entries for all the fields.
#[derive(Clap, Debug)]
pub struct PankitGetNote {
    /// The path of the anki database to get available model and fields from.
    pub database: PathBuf,
    /// Path to a pundit note. If this note contains anki notes, the model and deck from the first note will be used.
    /// If the note does not contain a pankit block, pundit will ask for the model and deck interactively.
    pub model_filename: Option<PathBuf>
}

#[derive(Clap, Debug)]
pub enum ConflictHandling {
    /// Show an error if any conflict is encountered. Do not change anything in the database
    GiveError,
    /// Print out all conflicts but apply any other changes nonetheless
    Ignore,
    /// Blindly use the contents from anki
    Anki,
    /// Blindly use the contents from pundit
    Pundit,
}
