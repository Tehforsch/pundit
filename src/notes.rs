use crate::config::NOTE_EXTENSION;
use crate::note::{get_link_filenames, Note};
use anyhow::{anyhow, Context, Result};
use generational_arena::{Arena, Index};
use std::fs;
use std::path::Path;

pub struct Notes {
    pub arena: Arena<Note>,
}

impl Notes {
    pub fn iter(&self) -> impl Iterator<Item = &Note> {
        self.arena.iter().map(|(_, note)| note)
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn get(&self, index: Index) -> &Note {
        self.arena.get(index).unwrap()
    }

    pub fn find_by_filename(&self, filename: &Path) -> Option<&Note> {
        self.iter()
            .find(|n| n.filename.file_name().unwrap() == filename)
    }
}

pub fn read_notes(note_folder: &Path) -> Result<Notes> {
    let mut arena = Arena::new();
    let mut indices = vec![];
    for entry in fs::read_dir(note_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == NOTE_EXTENSION {
                    let note = Note::from_filename_no_links(&path)?;
                    indices.push(arena.insert(note));
                }
            }
        }
    }
    for i in indices {
        set_links(i, &mut arena)?;
    }
    Ok(Notes { arena })
}

pub fn set_links(note_index: Index, notes: &mut Arena<Note>) -> Result<()> {
    let note = notes.get_mut(note_index).unwrap();
    let contents = note.get_contents()?;
    let indices: Vec<Index> = notes
        .iter()
        .map(|(i, _)| i)
        .filter(|&i| i != note_index)
        .collect();

    for link in get_link_filenames(&contents) {
        let mut found = false;
        for i in indices.iter() {
            let (mut_note, n) = notes.get2_mut(note_index, *i);
            if link == n.unwrap().filename.file_name().unwrap() {
                found = true;
                mut_note.unwrap().links.push(*i);
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
