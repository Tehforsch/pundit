use clap::Clap;

use crate::note_arg::NoteArg;

/// Open notes from a journal - date based notes.
#[derive(Clap, Debug)]
pub struct JournalOpts {
    /// Name of the journal to open
    pub name: String,
    /// Print the date of the target journal note, not the filename
    #[clap(short, long)]
    pub date: bool,

    #[clap(subcommand)]
    pub subcmd: JournalSubCommand,
}

#[derive(Clap, Debug)]
pub enum JournalSubCommand {
    // Find,
    /// Open the note for yesterday
    Yesterday,
    /// Open the note for today
    Today,
    /// Open the note for tomorrow
    Tomorrow,
    /// Given a journal entry, open the previous entry - skip all intermediate days that do not have an entry.
    /// When no previous entry exists return the given entry and do not create a new one.
    Previous(NoteArg),
    /// Given a journal entry, open the next entry - skip all intermediate days that do not have an entry.
    /// When no next entry exists return the given entry and do not create a new one.
    Next(NoteArg),
    /// Given a journal entry, open an entry for the previous day, whether it exists or not.
    DayBefore(NoteArg),
    /// Given a journal entry, open an entry for the next day, whether it exists or not.
    DayAfter(NoteArg),
}
