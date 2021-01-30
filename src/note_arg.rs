use std::path::PathBuf;

use clap::Clap;

use crate::{note::Note, notes::Notes};

#[derive(Clap, Debug)]
pub struct NoteArg {
    pub filename: PathBuf,
}

impl NoteArg {
    pub fn find_in<'a>(&self, notes: &'a Notes) -> Option<&'a Note> {
        notes.iter().filter(|n| n.filename == self.filename).next()
    }
}
