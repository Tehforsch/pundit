use crate::Note;
use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use crate::anki::anki_card::AnkiCard;
use crate::anki::anki_collection::AnkiCollection;
use crate::anki::anki_note::AnkiNote;
use crate::anki::AnkiNoteInfo;
use crate::anki::{
    add_anki_card, add_anki_note, anki_note_is_in_collection, close_connection,
    get_new_anki_note_and_cards, read_collection, read_notes,
};

pub fn update_anki(path: &Path, notes: &[Note]) -> Result<()> {
    let connection = Connection::open(path).unwrap();
    let anki_notes = read_notes(&connection)?;
    let collection = read_collection(&connection)?;
    update_and_add_anki_notes_from_pundit_contents(&connection, notes, &collection, &anki_notes)?;
    close_connection(connection)?;
    Ok(())
}

pub fn update_and_add_anki_notes_from_pundit_contents(
    connection: &Connection,
    notes: &[Note],
    collection: &AnkiCollection,
    anki_notes: &[AnkiNote],
) -> Result<()> {
    let anki_notes_and_cards = get_anki_notes_and_cards_for_pundit_notes(collection, notes)?;
    let mut num_notes_added = 0;
    let mut num_cards_added = 0;
    let mut num_notes_ignored = 0;
    for (anki_note, anki_cards) in anki_notes_and_cards.iter() {
        if !anki_note_is_in_collection(anki_notes, anki_note) {
            num_notes_added += add_anki_note(&connection, anki_note)
                .context(format!("While adding anki note {}", anki_note.id))?;
            for anki_card in anki_cards {
                num_cards_added +=
                    add_anki_card(&connection, anki_card).context("While adding anki card")?;
            }
        } else {
            num_notes_ignored += 1;
        }
    }
    println!(
        "Ignored {} notes which are already in the database.",
        num_notes_ignored
    );
    println!("Added {} notes.", num_notes_added);
    println!("Added {} cards.", num_cards_added);
    Ok(())
}

pub fn get_anki_notes_and_cards_for_pundit_notes(
    collection: &AnkiCollection,
    notes: &[Note],
) -> Result<Vec<(AnkiNote, Vec<AnkiCard>)>> {
    let mut results = vec![];
    for pundit_note in notes.iter() {
        results.extend(get_anki_notes_and_cards_for_pundit_note(
            collection,
            pundit_note,
        )?)
    }
    Ok(results)
    // let res: Vec<Vec<(AnkiNote, Vec<AnkiCard>)>> = notes
    //     .iter()
    //     .map(|pundit_note| get_anki_notes_and_cards_for_pundit_note(collection, pundit_note))
    //     .flatten()
    //     .collect();
}

fn get_anki_notes_and_cards_for_pundit_note(
    collection: &AnkiCollection,
    pundit_note: &Note,
) -> Result<Vec<(AnkiNote, Vec<AnkiCard>)>> {
    get_anki_info_for_pundit_note(pundit_note)
        .context(format!(
            "While reading anki entries from note {}",
            pundit_note.title
        ))?
        .iter()
        .map(|anki_note_info| get_new_anki_note_and_cards(collection, anki_note_info))
        .collect()
}

fn get_anki_info_for_pundit_note(pundit_note: &Note) -> Result<Vec<AnkiNoteInfo>> {
    let re = Regex::new(r"#anki (\d+) ([a-zA-Z]+) ([a-zA-Z:]+)").unwrap();
    let lines = pundit_note
        .get_lines()
        .context("While reading file contents")?;
    let mut currently_at_anki_entry = false;
    let mut note_info: Option<AnkiNoteInfo> = None;
    let mut res = vec![];
    for line in lines {
        if currently_at_anki_entry {
            match scan_for_anki_fields(&line?) {
                None => {
                    currently_at_anki_entry = false;
                    res.push(note_info.unwrap());
                    note_info = None;
                }
                Some((key, value)) => {
                    note_info.as_mut().unwrap().fields.insert(key, value);
                }
            }
        } else {
            let x = line?;
            match re.captures(&x) {
                None => {}
                Some(capture) => {
                    note_info = Some(AnkiNoteInfo {
                        fields: HashMap::new(),
                        id: i64::from_str(&capture[1])
                            .context("While reading note id as integer")?,
                        model_name: capture[2].to_string(),
                        deck_name: capture[3].to_string(),
                    });
                    currently_at_anki_entry = true;
                }
            }
        }
    }
    if let Some(info) = note_info {
        // In case that the last anki entry ended with the last line
        res.push(info);
    }
    Ok(res)
}
fn scan_for_anki_fields(line: &str) -> Option<(String, String)> {
    let re = Regex::new(r"#([a-zA-Z]+) (.*)").unwrap();
    re.captures(line)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
}
