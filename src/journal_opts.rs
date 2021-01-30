use crate::note_arg::NoteArg;
use clap::Clap;

/// Various functions to deal with date based notes
#[derive(Clap, Debug)]
pub struct JournalOpts {
    pub name: String,

    #[clap(subcommand)]
    pub subcmd: JournalSubCommand,
}

#[derive(Clap, Debug)]
pub enum JournalSubCommand {
    // Find,
    Yesterday,
    Today,
    Tomorrow,
    Previous(NoteArg),
    // Next,
    // DayBefore,
    // DayAfter,
}
