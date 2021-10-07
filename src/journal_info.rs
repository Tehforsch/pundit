use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use chrono::NaiveDate;

use crate::config;
use crate::dir_utils::create_folder;
use crate::note::Note;
use crate::note_utils::find_or_create_note;
use crate::notes::Notes;

pub struct JournalInfo {
    pub name: String,
    pub folder: PathBuf,
}

impl JournalInfo {
    pub fn from_name(notes: &mut Notes, name: &str) -> Result<JournalInfo> {
        let folder = get_folder(&notes.folder, name);
        ensure_journal_folder_exists(&folder)?;
        ensure_journal_base_note_exists(notes, &folder, name)?;
        Ok(JournalInfo {
            name: name.to_owned(),
            folder: folder.clone(),
            // base_note: get_journal_base_note(notes, &folder, name)?,
        })
    }

    pub fn get_link_text_to_base_note<'a>(&self, notes: &'a Notes) -> Result<String> {
        let base_note = self.get_base_note(notes);
        Ok(format!(
            "\n{}",
            base_note.get_link_from_folder(&self.folder)?
        ))
    }

    pub fn get_note_title_from_date(&self, date: &NaiveDate) -> String {
        assert_eq!(config::JOURNAL_TITLE_FORMAT, "{} {}");
        format!(
            "{} {}",
            self.name,
            date.format(config::JOURNAL_DATE_FORMAT_STR)
        )
    }

    pub fn get_date_from_note(note: &Note) -> Result<NaiveDate> {
        JournalInfo::get_date_from_note_title(&note.title)
    }

    fn get_date_from_note_title(title: &str) -> Result<NaiveDate> {
        assert_eq!(config::JOURNAL_TITLE_FORMAT, "{} {}"); // This needs to be changed when the format string changes.
        let date_part_of_title = title.split_once(" ").map(|(_, s1)| s1);
        date_part_of_title
            .ok_or_else(|| {
                anyhow!(format!(
                    "Invalid format in title for journal note: {}",
                    title
                ))
            })
            .and_then(|date_str| {
                NaiveDate::parse_from_str(date_str, config::JOURNAL_DATE_FORMAT_STR)
                    .context(format!("Invalid date in title for journal note: {}", title))
            })
    }

    pub fn get_base_note<'a>(&self, notes: &'a Notes) -> &'a Note {
        notes.find_by_title(&self.name).unwrap()
    }
}

fn get_folder<'a>(base_folder: &'a Path, journal_name: &'a str) -> PathBuf {
    let folder = match config::JOURNAL_IN_SUBFOLDERS {
        true => base_folder.join(journal_name),
        false => base_folder.to_path_buf(),
    };
    folder
}

fn ensure_journal_folder_exists(folder: &Path) -> Result<()> {
    if !folder.exists() {
        create_folder(folder)
    } else {
        Ok(())
    }
}

fn ensure_journal_base_note_exists<'a>(
    notes: &'a mut Notes,
    journal_folder: &Path,
    name: &str,
) -> Result<()> {
    find_or_create_note(notes, &journal_folder, &name)?;
    Ok(())
}
