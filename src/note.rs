use crate::config::{LINK_FORMAT, NOTE_DATE_STR_FORMAT, NOTE_FILENAME_STR_FORMAT, TITLE_STRING};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};

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
    pub fn from_filename(filename: &Path) -> Result<Note> {
        let contents = fs::read_to_string(filename)?;
        Ok(Note {
            filename: filename.to_path_buf(),
            title: get_title(&contents)
                .context(format!("Opening {}", filename.to_str().unwrap()))?,
            links: get_links(&contents)?,
        })
    }

    pub fn from_title_and_date(title: &str) -> Note {
        let date_time = Local::now();
        let filename = Path::new(&get_filename_from_title(&title, date_time)).to_path_buf();
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

    pub fn get_contents(&self) -> Result<String> {
        fs::read_to_string(&self.filename).context("While reading file")
    }

    pub fn get_link(&self) -> String {
        LINK_FORMAT
            .replace("{filename}", &self.filename.to_str().unwrap())
            .replace("{title}", &self.title)
    }
}

fn get_title(contents: &str) -> Result<String> {
    if !contents.starts_with(TITLE_STRING) {
        Err(anyhow!("Note does not contain title"))
    } else {
        let title = contents
            .lines()
            .next()
            .unwrap()
            .strip_prefix(TITLE_STRING)
            .unwrap();
        Ok(title.to_string())
    }
}

fn get_filename_from_title(title: &str, date_time: DateTime<Local>) -> String {
    let title_string = title.replace(" ", "_");
    let date_string = format!("{}", date_time.format(NOTE_DATE_STR_FORMAT));
    NOTE_FILENAME_STR_FORMAT
        .replace("{titleString}", &title_string)
        .replace("{dateString}", &date_string)
}

fn get_links(contents: &str) -> Result<Vec<Note>> {
    let re = Regex::new(r"\[\[file:(.*?)\]\[(.*?)\]\]").unwrap();
    let mut links = vec![];
    for cap in re.captures_iter(contents) {
        links.push(Note {
            filename: Path::new(&cap[1].to_string()).to_path_buf(),
            title: cap[2].to_string(),
            links: vec![],
        });
    }
    Ok(links)
}

fn get_title_string(title: &str) -> String {
    format!("#+TITLE: {}", title)
}

// fn get_link_string(filename: String, title: String) -> String {
//     format!("[file:{}][{}]", filename, title)
// }
