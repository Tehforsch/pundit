use std::fs;
use std::path::{Path, PathBuf};
use std::io;

pub mod config;
pub mod note;

use crate::note::Note;

fn main() -> io::Result<()> {
    let note_folder = Path::new("test");
    let notes = read_notes(&note_folder)?;
    println!("All notes: ");
    list_notes(&notes);
    for note in notes.iter() {
        println!("Backlinks for {}:", note.title);
        list_backlinks(&notes, &note);
    }
    Ok(())
}

fn read_notes(note_folder: &Path) -> io::Result<Vec<Note>> {
    let mut notes = vec![];
    for entry in fs::read_dir(note_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            notes.push(Note::from_filename(&path));
        }
    }
    Ok(notes)
}

fn list_notes(notes: &Vec<Note>) {
    for note in notes.iter() {
        println!("{}", note.title);
    }
}

fn list_backlinks(notes: &Vec<Note>, note: &Note) {
    for note in notes.iter() {
        for link in note.links.iter() {
            if link.title == note.title {
                println!("{}", link.title);
            }
        }
    }
}
