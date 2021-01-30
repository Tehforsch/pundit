use std::path::Path;

use anyhow::Result;

use crate::{
    file_utils::append_to_file,
    note::{create_new_note_from_title, Note},
    notes::Notes,
};

pub fn get_backlinks<'a>(notes: &'a Notes, note: &'a Note) -> impl Iterator<Item = &'a Note> {
    note.backlinks.iter().map(move |link| &notes[*link])
}

pub enum FindNoteResult<'a> {
    New(&'a Note),
    Existing(&'a Note),
}

impl<'a> FindNoteResult<'a> {
    pub fn unwrap(&self) -> &'a Note {
        match self {
            Self::New(n) => n,
            Self::Existing(n) => n,
        }
    }
}

pub fn find_or_create_note<'a>(
    notes: &'a mut Notes,
    folder: &Path,
    title: &str,
) -> Result<FindNoteResult<'a>> {
    let mb_index = notes.find_index_by_title(title);
    if let Some(index) = mb_index {
        Ok(FindNoteResult::Existing(&notes[index]))
    } else {
        let new_note = create_new_note_from_title(notes, folder, &title)?;
        let note_index = notes.push(new_note);
        Ok(FindNoteResult::New(&notes[note_index]))
    }
}

pub fn find_or_create_note_with_special_content<'a>(
    notes: &'a mut Notes,
    folder: &Path,
    title: &str,
    content: &str,
) -> Result<&'a Note> {
    Ok(match find_or_create_note(notes, folder, title)? {
        FindNoteResult::New(note) => {
            append_to_file(&note.filename, &content)?;
            note
        }
        FindNoteResult::Existing(note) => note,
    })
}
