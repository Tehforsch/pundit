use crate::{journal_info::JournalInfo, journal_opts::JournalOpts, note::Note, note_utils::find_or_create_note_with_special_content};
use crate::{journal_opts::JournalSubCommand, note_arg::NoteArg};
use crate::{note_utils::get_backlinks, notes::Notes};

use log::info;
use anyhow::{anyhow, Context, Result};
use chrono::{Duration, Local, NaiveDate};

pub fn run_journal(notes: &mut Notes, args: &JournalOpts) -> Result<()> {
    let journal_info = JournalInfo::from_name(notes, &args.name)?;
    let target_date = match &args.subcmd {
        // args::JournalSubCommand::Find => {}
        JournalSubCommand::Yesterday => get_date_yesterday()?,
        JournalSubCommand::Today => get_date_today()?,
        JournalSubCommand::Tomorrow => get_date_tomorrow()?,
        JournalSubCommand::Previous(n) => {
            get_date_via_selector(notes, &journal_info, n, previous_date)?
        }
        JournalSubCommand::Next(n) => get_date_via_selector(notes, &journal_info, n, next_date)?,
        JournalSubCommand::DayBefore(n) => {
            get_date_via_selector(notes, &journal_info, n, day_before_date)?
        }
        JournalSubCommand::DayAfter(n) => {
            get_date_via_selector(notes, &journal_info, n, day_after_date)?
        }
    };
    let note = find_or_create_journal_note_for_date(notes, &journal_info, &target_date)?;
    match args.date {
        false => note.show_filename(),
        true => info!("{}", target_date),
    };
    Ok(())
}

fn find_or_create_journal_note_for_date<'a>(
    notes: &'a mut Notes,
    journal: &JournalInfo,
    date_time: &NaiveDate,
) -> Result<&'a Note> {
    let title = journal.get_note_title_from_date(date_time);
    let link_text = &journal.get_link_text_to_base_note(notes)?;
    let target_note =
        find_or_create_note_with_special_content(notes, &journal.folder, &title, link_text)?;
    Ok(target_note)
}

fn get_date_yesterday() -> Result<NaiveDate> {
    get_date_duration(Duration::days(-1))
}

fn get_date_today() -> Result<NaiveDate> {
    get_date_duration(Duration::days(0))
}

fn get_date_tomorrow() -> Result<NaiveDate> {
    get_date_duration(Duration::days(1))
}

fn get_date_duration(duration: Duration) -> Result<NaiveDate> {
    Local::now()
        .naive_local()
        .checked_add_signed(duration)
        .map(|date_time| date_time.date())
        .context("Date overflow")
}

fn get_date_via_selector<'a, F>(
    notes: &'a Notes,
    journal: &JournalInfo,
    note_name: &NoteArg,
    select: F,
) -> Result<NaiveDate>
where
    F: Fn(&Vec<NaiveDate>, &NaiveDate) -> NaiveDate,
{
    let dates = get_all_entry_dates(notes, journal)?;
    let input_note = note_name
        .find_in(notes)
        .ok_or_else(|| anyhow!("Input note not found: {:?}", note_name))?;
    let input_date = JournalInfo::get_date_from_note(input_note)?;
    Ok(select(&dates, &input_date))
}

fn get_all_entry_dates(notes: &Notes, journal: &JournalInfo) -> Result<Vec<NaiveDate>> {
    let journal_base_note = journal.get_base_note(notes);
    get_backlinks(notes, journal_base_note)
        .map(|note| JournalInfo::get_date_from_note(note))
        .collect()
}

fn previous_date(entry_dates: &Vec<NaiveDate>, date: &NaiveDate) -> NaiveDate {
    let previous_entry = entry_dates.iter().filter(|d| d < &date).max().clone();
    previous_entry.unwrap_or(date).clone()
}

fn next_date(entry_dates: &Vec<NaiveDate>, date: &NaiveDate) -> NaiveDate {
    let previous_entry = entry_dates.iter().filter(|d| d > &date).min().clone();
    previous_entry.unwrap_or(date).clone()
}

fn day_before_date(_entry_dates: &Vec<NaiveDate>, date: &NaiveDate) -> NaiveDate {
    date.checked_add_signed(Duration::days(-1)).unwrap()
}

fn day_after_date(_entry_dates: &Vec<NaiveDate>, date: &NaiveDate) -> NaiveDate {
    date.checked_add_signed(Duration::days(1)).unwrap()
}
