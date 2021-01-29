use crate::{
    config::{LINK_FORMAT, NOTE_DATE_FORMAT_STR, NOTE_FILENAME_STR_FORMAT, TITLE_STRING},
    dir_utils::get_relative_path,
    notes::Notes,
};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use generational_arena::Index;
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
struct InvalidNoteError;
#[derive(Debug, Clone)]
struct InvalidTitleError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub filename: PathBuf,
    pub title: String,
    pub links: Vec<Index>,
    pub backlinks: Vec<Index>,
}

impl Note {
    pub fn from_filename_no_links(filename: &Path) -> Result<Note> {
        let contents = Note::get_first_line(filename)
            .context(format!("Reading contents of {:?}", filename))?;
        Ok(Note {
            filename: filename.to_owned(),
            title: get_title(&contents)
                .context(format!("Opening {}", filename.to_str().unwrap()))?,
            links: vec![],
            backlinks: vec![],
        })
    }

    pub fn from_folder_and_title(folder: &Path, title: &str) -> Note {
        let date_time = Local::now();
        let filename = Path::new(folder)
            .join(&get_filename_from_title(&title, date_time))
            .to_path_buf();
        Note::empty_note(filename, title)
    }

    fn empty_note(filename: PathBuf, title: &str) -> Note {
        Note {
            filename: filename,
            title: title.to_string(),
            links: vec![],
            backlinks: vec![],
        }
    }

    pub fn write_without_contents(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.filename)?;
        let contents = get_title_string(&self.title);
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn get_first_line(filename: &Path) -> Result<String> {
        let file = fs::File::open(filename)?;
        let mut buffer = BufReader::new(file);
        let mut first_line = String::new();
        buffer.read_line(&mut first_line)?;
        Ok(first_line)
    }

    pub fn get_contents(&self) -> Result<String> {
        fs::read_to_string(&self.filename).context("While reading file")
    }

    pub fn get_link_from(&self, note1: &Note) -> Result<String> {
        let relative_path = get_relative_path(&self.filename, note1.filename.parent().unwrap())?;
        Ok(LINK_FORMAT
            .replace("{relative_path}", relative_path.to_str().unwrap())
            .replace("{title}", &self.title))
    }

    pub fn show_filename(&self) {
        println!(
            "{}",
            self.filename.canonicalize().unwrap().to_str().unwrap()
        );
    }
}

fn get_title(first_line: &str) -> Result<String> {
    if !first_line.starts_with(TITLE_STRING) {
        Err(anyhow!("Note does not contain title"))
    } else {
        let title = first_line
            .strip_prefix(TITLE_STRING)
            .ok_or_else(|| anyhow!(format!("Invalid title string: {}", first_line)))?
            .trim_end_matches("\n");
        Ok(title.to_string())
    }
}

pub fn get_link_filenames(contents: &str) -> Vec<PathBuf> {
    let re = Regex::new(r"\[\[file:(.*?)\]\[(.*?)\]\]").unwrap();
    re.captures_iter(contents)
        .map(|cap| Path::new(&cap[1].to_string()).to_path_buf())
        .collect()
}

fn get_filename_from_title(title: &str, date_time: DateTime<Local>) -> String {
    let title_string = title.replace(" ", "_");
    let date_string = format!("{}", date_time.format(NOTE_DATE_FORMAT_STR));
    NOTE_FILENAME_STR_FORMAT
        .replace("{titleString}", &title_string)
        .replace("{dateString}", &date_string)
}

fn get_title_string(title: &str) -> String {
    format!("#+TITLE: {}", title)
}

pub fn find_or_create_note(notes: &Notes, folder: &Path, title: &str) -> Result<Note> {
    // Keep in mind this returns a note which means it clones any already existing note for simplicity.
    // Otherwise we would need to pass mutable references &mut Notes everywhere in order to add it
    // to the arena and get a &Note to the new Note that we just created.
    match notes.find_by_title(title) {
        Some(note) => Ok(note.clone()),
        None => create_new_note_from_title(notes, folder, &title),
    }
}

pub fn create_new_note_from_title(notes: &Notes, folder: &Path, title: &str) -> Result<Note> {
    match notes.find_by_title(title) {
        Some(n) => {
            return Err(anyhow!(
                "Note with title: {} already exists at {:?}",
                title,
                n.filename
            ))
        }
        None => {}
    };
    let note = Note::from_folder_and_title(folder, title);
    if note.filename.exists() {
        return Err(anyhow!(
            "Note with filename {:?} already exists",
            note.filename
        ));
    }
    note.write_without_contents()
        .context("Failed to write note")?;
    Ok(note)
}

// fn get_link_string(filename: String, title: String) -> String {
//     format!("[file:{}][{}]", filename, title)
// }
