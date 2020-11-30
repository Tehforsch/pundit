use clap::Clap;
use std::path::PathBuf;
use std::str::FromStr;

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
    /// Run on note setups with multiple directories. This will traverse the entire given directory tree recursively and look for notes.
    #[clap(short, long)]
    pub multidir: bool,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    List(ListNotes),
    Link(GetLinkText),
    ListBacklinks(ListBacklinks),
    Backlinks(FindBacklinks),
    Find(FindNoteInteractively),
    Delete(DeleteNote),
    Rename(RenameNote),
    Pankit(Pankit),
    PankitGetNote(PankitGetNote),
    ListGraph(ListGraph),
    Graph(FindGraph),
}

/// List notes.
#[derive(Clap, Debug)]
pub struct ListNotes {
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
}

/// Interactively choose a note and then display a properly formatted link for it
#[derive(Clap, Debug)]
pub struct GetLinkText {
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
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
    /// Optional: Only list notes which contain this string in the title
    pub filter: Option<String>,
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
    // #[clap(default_value = "error")]
    /// How to deal with conflicting contents between anki and pundit that cannot be resolved automatically
    #[clap(possible_values = &["ignore", "error", "pundit", "anki"], default_value = "error")]
    pub conflict_handling: ConflictHandling,
}

/// Add a pankit note by generating an id, allowing to interactively select model/deck and adding empty entries for all the fields.
#[derive(Clap, Debug)]
pub struct PankitGetNote {
    /// The path of the anki database to get available model and fields from.
    pub database: PathBuf,
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

// Implement the trait
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
