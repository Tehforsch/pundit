use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config,
    file_utils::append_to_file,
    note::{create_new_note_from_title, Note},
};
use crate::{note::find_or_create_note, notes::Notes};

use super::args;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local};

pub fn run_journal(notes: &Notes, base_folder: &Path, args: &args::Journal) -> Result<()> {
    match args.subcmd {
        // args::JournalSubCommand::Find => {}
        args::JournalSubCommand::Yesterday => find_yesterday(notes, base_folder, &args.name),
        args::JournalSubCommand::Today => find_today(notes, base_folder, &args.name),
        args::JournalSubCommand::Tomorrow => find_tomorrow(notes, base_folder, &args.name),
        // args::JournalSubCommand::Previous => {}
        // args::JournalSubCommand::Next => {}
        // args::JournalSubCommand::DayBefore => {}
        // args::JournalSubCommand::DayAfter => {}
    }
}

fn find_yesterday(notes: &Notes, base_folder: &Path, journal_name: &str) -> Result<()> {
    find_duration(notes, base_folder, journal_name, Duration::days(-1))
}

fn find_today(notes: &Notes, base_folder: &Path, journal_name: &str) -> Result<()> {
    find_duration(notes, base_folder, journal_name, Duration::days(0))
}

fn find_tomorrow(notes: &Notes, base_folder: &Path, journal_name: &str) -> Result<()> {
    find_duration(notes, base_folder, journal_name, Duration::days(1))
}

fn find_duration(
    notes: &Notes,
    base_folder: &Path,
    journal_name: &str,
    duration: Duration,
) -> Result<()> {
    let date_time = Local::now()
        .checked_add_signed(duration)
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
        None => create_new_journal_note_from_title(notes, &folder, &title, journal_name)?
            .show_filename(),
    };
    Ok(())
}

fn create_new_journal_note_from_title(
    notes: &Notes,
    folder: &Path,
    title: &str,
    journal_name: &str,
) -> Result<Note> {
    let note = create_new_note_from_title(notes, folder, title)?;
    append_link_to_main_journal_note(notes, &note, folder, journal_name)?;
    Ok(note)
}

fn append_link_to_main_journal_note(
    notes: &Notes,
    note: &Note,
    folder: &Path,
    journal_name: &str,
) -> Result<()> {
    let main_note = find_or_create_note(notes, folder, journal_name)?; // Yes, this note should have the journal name as the title!
    let link_text = format!("\n{}", main_note.get_link_from(note)?);
    append_to_file(&note.filename, &link_text)?;
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
