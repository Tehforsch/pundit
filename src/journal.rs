use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::notes::Notes;
use crate::{config, note::create_new_note_from_title};

use super::args;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local};

pub fn run_journal(notes: &Notes, base_folder: &Path, args: &args::Journal) -> Result<()> {
    match args.subcmd {
        // args::JournalSubCommand::Find => {}
        args::JournalSubCommand::Yesterday => run_yesterday(notes, base_folder, &args.name),
        // args::JournalSubCommand::Today => {}
        // args::JournalSubCommand::Tomorrow => {}
        // args::JournalSubCommand::Previous => {}
        // args::JournalSubCommand::Next => {}
        // args::JournalSubCommand::DayBefore => {}
        // args::JournalSubCommand::DayAfter => {}
    }
}

fn run_yesterday(notes: &Notes, base_folder: &Path, journal_name: &str) -> Result<()> {
    let date_time = Local::now()
        .checked_sub_signed(Duration::days(1))
        .context("Date overflow")?;
    find_or_create_note_for_date_and_print_filename(notes, base_folder, journal_name, &date_time)?;
    Ok(())
}

fn find_or_create_note_for_date_and_print_filename(
    notes: &Notes,
    base_folder: &Path,
    journal_name: &str,
    date_time: &DateTime<Local>,
) -> Result<()> {
    let title = get_journal_note_title(journal_name, date_time);
    let folder = get_folder(base_folder, journal_name);
    create_folder_if_nonexistent(&folder)?;
    let mb_note = notes.find_by_title(&title);
    match mb_note {
        Some(note) => note.show_filename(),
        None => create_new_note_from_title(notes, &folder, &title)?.show_filename(),
    };
    Ok(())
}

fn get_folder<'a>(base_folder: &'a Path, journal_name: &'a str) -> PathBuf {
    let folder = match config::JOURNAL_IN_SUBFOLDERS {
        true => base_folder.join(journal_name),
        false => base_folder.to_path_buf(),
    };
    folder
}

fn create_folder_if_nonexistent(folder: &Path) -> Result<()> {
    fs::create_dir_all(folder).context("While creating folder")
}

fn get_journal_note_title(journal_name: &str, date_time: &DateTime<Local>) -> String {
    format!(
        "{}_{}",
        journal_name,
        date_time.date().format(config::JOURNAL_DATE_FORMAT_STR)
    )
}
