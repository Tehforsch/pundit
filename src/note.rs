use crate::config::COMMENT_STRING;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct InvalidNoteError;
#[derive(Debug, Clone)]
struct InvalidTitleError;

#[derive(Debug, Clone)]
pub struct Note {
    pub filename: PathBuf,
    pub title: String,
    pub links: Vec<Note>,
}

impl Note {
    pub fn from_filename(filename: &Path) -> Note {
        let contents = fs::read_to_string(filename).expect("Could not read file");
        Note {
            filename: filename.to_path_buf(),
            title: get_title(&contents).unwrap(),
            links: get_links(&contents),
        }
    }

    pub fn from_title(title: &str) -> Note {
        let filename = Path::new(&get_filename_from_title(&title)).to_path_buf();
        Note {
            filename,
            title: title.to_string(),
            links: vec![],
        }
    }

    pub fn write_without_contents(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.filename)?;
        let contents = get_title_string(&self.title);
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

fn get_title(contents: &str) -> Result<String, InvalidNoteError> {
    if !contents.starts_with(COMMENT_STRING) {
        Err(InvalidNoteError)
    } else {
        let title = contents
            .lines()
            .next()
            .unwrap()
            .strip_prefix(COMMENT_STRING)
            .unwrap();
        Ok(title.to_string())
    }
}

fn get_filename_from_title(title: &str) -> String {
    title.replace(" ", "_") + ".org"
}

fn get_links(contents: &str) -> Vec<Note> {
    let re = Regex::new(r"\[\[file:(.*)\]\[(.*?)\]\]").unwrap();
    let mut links = vec![];
    for cap in re.captures_iter(contents) {
        links.push(Note {
            filename: Path::new(&cap[1].to_string()).to_path_buf(),
            title: cap[2].to_string(),
            links: vec![],
        });
    }
    links
}

fn get_title_string(title: &str) -> String {
    format!("#+TITLE: {}", title)
}

// fn get_link_string(filename: String, title: String) -> String {
//     format!("[file:{}][{}]", filename, title)
// }
