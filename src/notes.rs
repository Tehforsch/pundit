use crate::config::NOTE_EXTENSION;
use crate::dir_utils::get_files;
use crate::dir_utils::get_files_recursively;
use crate::note::{get_link_filenames, Note};
use anyhow::{anyhow, Context, Result};
use generational_arena::{Arena, Index};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Deserialize, Serialize)]
pub struct NotesDatabase {
    notes: Notes,
    modified_timestamp: Option<SystemTime>,
}

impl NotesDatabase {
    pub fn from_file(filename: &Path) -> Result<NotesDatabase> {
        let data = fs::read_to_string(filename).context("While reading pundit database")?;
        Ok(serde_yaml::from_str(&data).context("Reading pundit database contents")?)
    }

    pub fn to_file(&self, filename: &Path) -> Result<()> {
        let data = serde_yaml::to_string(self).context("While converting pundit db to yaml")?;
        fs::write(filename, data).context("Unable to write pundit db file")?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Notes {
    pub arena: Arena<Note>,
}

impl Notes {
    pub fn iter(&self) -> impl Iterator<Item = &Note> {
        self.arena.iter().map(|(_, note)| note)
    }

    pub fn index_iter(&self) -> impl Iterator<Item = (Index, &Note)> {
        self.arena.iter()
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn get(&self, index: Index) -> Option<&Note> {
        self.arena.get(index)
    }

    pub fn find_by_filename(&self, filename: &Path) -> Option<&Note> {
        self.iter().find(|n| n.filename == filename)
    }

    pub fn empty() -> Notes {
        Notes {
            arena: Arena::new(),
        }
    }
}

impl std::ops::Index<Index> for Notes {
    type Output = Note;

    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub fn read_notes_from_database(note_folder: &Path, db_path: &Path) -> Result<Notes> {
    let mb_notes_db = NotesDatabase::from_file(db_path);
    let mut notes_db = match mb_notes_db {
        Err(_) => NotesDatabase {
            modified_timestamp: None,
            notes: Notes::empty(),
        },
        Ok(notes_db) => notes_db,
    };
    for note in notes_db.notes.iter() {
        println!("{}", note.title);
    }
    update_database_from_db_file(note_folder, &mut notes_db)?;
    notes_db.to_file(db_path)?;
    Ok(notes_db.notes)
}

fn update_database_from_db_file(note_folder: &Path, notes_db: &mut NotesDatabase) -> Result<()> {
    for entry_res in fs::read_dir(note_folder)? {
        let entry = entry_res?;
        let modified_timestamp = entry.metadata()?.modified()?;
        if notes_db
            .modified_timestamp
            .map_or(true, |db_timestamp| modified_timestamp > db_timestamp)
        {
            dbg!(&entry);
        }
    }
    Ok(())
}

pub fn read_notes(note_folder: &Path, database: &Option<PathBuf>, multidir: bool) -> Result<Notes> {
    match database {
        None => read_notes_from_folder(note_folder, multidir),
        Some(db_path) => read_notes_from_database(note_folder, db_path),
    }
}

fn get_files_maybe_recursively(note_folder: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    match recursive {
        true => get_files_recursively(note_folder),
        false => get_files(note_folder),
    }
}

pub fn read_notes_from_folder(note_folder: &Path, multidir: bool) -> Result<Notes> {
    let mut arena = Arena::new();
    let mut indices = vec![];
    for file in get_files_maybe_recursively(note_folder, multidir)? {
        if let Some(extension) = file.extension() {
            if extension == NOTE_EXTENSION {
                let note = Note::from_filename_no_links(&file.canonicalize()?)?;
                indices.push(arena.insert(note));
            }
        }
    }
    for i in indices {
        set_links(&mut arena, i)?;
    }
    Ok(Notes { arena })
}

pub fn set_links(notes: &mut Arena<Note>, note_index: Index) -> Result<()> {
    let note = notes.get_mut(note_index).unwrap();
    let cloned_filename = note.filename.clone();
    let parent_dir = cloned_filename
        .parent()
        .expect(&format!("Invalid filename for note: {:?}", &note.filename));
    let contents = note.get_contents().context("While reading note contents")?;
    let indices: Vec<Index> = notes
        .iter()
        .map(|(i, _)| i)
        .filter(|&i| i != note_index)
        .collect();

    for relative_link in get_link_filenames(&contents) {
        let link = parent_dir.join(relative_link).canonicalize()?;
        let mut found = false;
        for i in indices.iter() {
            let (n1, n2) = notes.get2_mut(note_index, *i);
            let note1 = n1.unwrap();
            let note2 = n2.unwrap();
            if link == note2.filename {
                found = true;
                note1.links.push(*i);
                note2.backlinks.push(note_index);
            }
        }
        if !found {
            Err(anyhow!(format!(
                "Invalid link in file: {}",
                link.to_str().unwrap()
            )))?;
        }
    }
    Ok(())
}
