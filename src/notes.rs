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

pub fn read_notes_from_database(
    note_folder: &Path,
    db_path: &Path,
    multidir: bool,
) -> Result<Notes> {
    let mb_notes_db = NotesDatabase::from_file(db_path);
    let mut notes_db = match mb_notes_db {
        Err(_) => NotesDatabase {
            modified_timestamp: None,
            notes: Notes::empty(),
        },
        Ok(notes_db) => notes_db,
    };
    update_database_from_db_file(note_folder, &mut notes_db, multidir)?;
    notes_db.to_file(db_path)?;
    Ok(notes_db.notes)
}

fn filter_result<F, T>(it: Box<dyn Iterator<Item = T>>, predicate: F) -> Result<Vec<T>>
where
    F: Fn(&T) -> Result<bool>,
{
    let results: Result<Vec<(T, bool)>> = it
        .map(move |t| match predicate(&t) {
            Err(e) => Err(e),
            Ok(v) => Ok((t, v)),
        })
        .collect();
    Ok(results?
        .into_iter()
        .filter(|(_, v)| *v)
        .map(|(t, _)| t)
        .collect())
}

fn get_newer_files_maybe_recursively(
    note_folder: &Path,
    notes_db: &mut NotesDatabase,
    multidir: bool,
) -> Result<Vec<PathBuf>> {
    let files = get_files_maybe_recursively(note_folder, multidir)?;
    filter_result(Box::new(files.into_iter()), |path| {
        Ok(match notes_db.modified_timestamp {
            Some(db_timestamp) => path.metadata()?.modified()? > db_timestamp,
            None => true,
        })
    })
}

fn update_database_from_db_file(
    note_folder: &Path,
    notes_db: &mut NotesDatabase,
    multidir: bool,
) -> Result<()> {
    for file in get_newer_files_maybe_recursively(note_folder, notes_db, multidir)? {
        dbg!(&file);
    }
    Ok(())
}

pub fn read_notes(note_folder: &Path, database: &Option<PathBuf>, multidir: bool) -> Result<Notes> {
    match database {
        None => read_notes_from_folder(note_folder, multidir),
        Some(db_path) => read_notes_from_database(note_folder, db_path, multidir),
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
