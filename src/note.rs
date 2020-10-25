use crate::config::COMMENT_STRING;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct InvalidNoteError;

#[derive(Debug)]
pub struct Note {
    pub filename: PathBuf,
    pub title: String,
    pub links: Vec<Note>
}

impl Note {
    pub fn from_filename(filename: &PathBuf) -> Note {
        let contents = fs::read_to_string(filename).expect("Could not read file");
        Note {
            filename: filename.clone(),
            title: get_title(&contents).unwrap(),
            links: get_links(&contents)
        }
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

fn get_links(contents: &str) -> Vec<Note> {
    let re = Regex::new(r"\[\[file:(.*)\]\[(.*?)\]\]").unwrap();
    let mut links = vec![];
    for cap in re.captures_iter(contents) {
        links.push(Note {
            filename: Path::new(&cap[1].to_string()).to_path_buf(),
            title: cap[2].to_string(),
            links: vec![]
        });
    }
    links
}
