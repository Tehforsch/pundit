use crate::journal_opts::JournalSubCommand;
use crate::notes::Notes;
use crate::{
    journal_info::JournalInfo, journal_opts::JournalOpts, note::Note,
    note_utils::find_or_create_note_with_special_content,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local};

pub fn run_journal(notes: &mut Notes, args: &JournalOpts) -> Result<()> {
    let journal_info = JournalInfo::from_name(notes, &args.name)?;
    let target_date = match &args.subcmd {
        // args::JournalSubCommand::Find => {}
        JournalSubCommand::Yesterday => get_date_yesterday()?,
        JournalSubCommand::Today => get_date_today()?,
        JournalSubCommand::Tomorrow => get_date_tomorrow()?,
        JournalSubCommand::Previous(_) => {
            todo!()
        } // args::JournalSubCommand::Previous(n) => {
          //     find_previous_entry(notes, base_folder, &args.name, n)
          // } // args::JournalSubCommand::Next => {}
          // args::JournalSubCommand::DayBefore => {}
          // args::JournalSubCommand::DayAfter => {}
    };
    let note = find_or_create_journal_note_for_date(notes, &journal_info, &target_date)?;
    note.show_filename();
    Ok(())
}

fn find_or_create_journal_note_for_date<'a>(
    notes: &'a mut Notes,
    journal: &JournalInfo,
    date_time: &DateTime<Local>,
) -> Result<&'a Note> {
    let title = journal.get_note_title_from_date(date_time);
    let link_text = &journal.get_link_text_to_base_note(notes)?;
    let target_note =
        find_or_create_note_with_special_content(notes, &journal.folder, &title, link_text)?;
    Ok(target_note)
    // Ok(match target_note {
    //     FindNoteResult::Existing(note) => note,
    //     FindNoteResult::New(note) => {
    //         append_link_to_main_journal_note(notes, journal, note);
    //         note
    //     }
    // })
}

fn get_date_yesterday() -> Result<DateTime<Local>> {
    get_date_duration(Duration::days(-1))
}

fn get_date_today() -> Result<DateTime<Local>> {
    get_date_duration(Duration::days(0))
}

fn get_date_tomorrow() -> Result<DateTime<Local>> {
    get_date_duration(Duration::days(1))
}

fn get_date_duration(duration: Duration) -> Result<DateTime<Local>> {
    Local::now()
        .checked_add_signed(duration)
        .context("Date overflow")
    // find_or_create_note_for_date_and_print_filename(notes, base_folder, journal_name, &date_time)?;
    // Ok(())
}

// fn find_previous_entry(
//     notes: &Notes,
//     base_folder: &Path,
//     journal_name: &str,
//     note_name: &NoteArg,
// ) -> Result<()> {
//     find_and_print_entry_via_selector(notes, base_folder, journal_name, note_name, previous_note)
// }

// fn find_and_print_entry_via_selector<'a, F>(
//     notes: &'a mut Notes,
//     base_folder: &Path,
//     journal_name: &str,
//     note_name: &NoteArg,
//     select: F,
// ) -> Result<()>
// where
//     F: Fn(&Vec<Index>, &Note) -> Result<&'a Note>,
// {
//     let journal_base_note = get_journal_base_note(notes, base_folder, journal_name)?;
//     let journal_note_indices = &journal_base_note.backlinks;
//     let input_note = note_name
//         .find_in(notes)
//         .ok_or_else(|| anyhow!("Input note not found: {:?}", note_name))?;
//     let selected_entry = select(journal_note_indices, input_note)?;
//     selected_entry.show_filename();
//     Ok(())
// }

// fn previous_note<'a>(notes: &Vec<Index>, note: &Note) -> Result<&'a Note> {
//     todo!()
// }

// fn create_new_journal_note_from_title(
//     notes: &Notes,
//     base_folder: &Path,
//     folder: &Path,
//     title: &str,
//     journal_name: &str,
// ) -> Result<Note> {
//     let note = create_new_note_from_title(notes, folder, title)?;
//     append_link_to_main_journal_note(notes, base_folder, &note, folder, journal_name)?;
//     Ok(note)
// }
