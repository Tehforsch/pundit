use crypto::digest::Digest;
use crypto::sha1::Sha1;
use rusqlite::{params, Connection};
use std::convert::TryFrom;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};

use crate::anki_card::AnkiCard;
use crate::anki_collection::AnkiCollection;
use crate::anki_deck::{get_anki_decks_from_json, AnkiDeck};
use crate::anki_model::{get_anki_models_from_json, AnkiModel};
use crate::anki_note::AnkiNote;

pub fn run_anki(path: &Path) -> Result<()> {
    let connection = Connection::open(path).unwrap();
    let _notes = read_notes(&connection)?;
    let collection = read_collection(&connection)?;
    let arbitrary_model_name = "Spanish";
    let arbitrary_deck_name = "Spanish";
    let (anki_note, anki_cards) = get_new_anki_note_and_cards(
        &collection,
        vec!["Some front".to_string(), "Some back".to_string()],
        arbitrary_model_name.to_string(),
        arbitrary_deck_name.to_string(),
    )
    .expect("Name does not exist.");
    let num_added = add_anki_note(&connection, anki_note)?;
    println!("Added {} notes.", num_added);
    let mut num_cards_added = 0;
    for anki_card in anki_cards {
        num_cards_added += add_anki_card(&connection, anki_card)?;
    }
    println!("Added {} cards.", num_cards_added);
    close_connection(connection)?;
    Ok(())
}

pub fn close_connection(connection: Connection) -> rusqlite::Result<()> {
    match connection.close() {
        Err((conn, _err)) => close_connection(conn),
        Ok(()) => Ok(()),
    }
}

pub fn get_csum(input: &str) -> i64 {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    i64::from_str_radix(&hasher.result_str()[..8], 16).expect("Invalid hash result")
}

fn add_note_id_as_field(fields: &mut Vec<String>, note_id: i64) {
    fields.insert(0, note_id.to_string());
}

pub fn get_new_anki_note_and_cards(
    collection: &AnkiCollection,
    fields: Vec<String>,
    model_name: String,
    deck_name: String,
) -> Result<(AnkiNote, Vec<AnkiCard>)> {
    let model = get_model_by_name(collection, model_name)?;
    let deck = get_deck_by_name(collection, deck_name)?;
    let anki_note = get_new_anki_note(fields, model);
    let anki_cards = get_new_anki_cards(model, deck, &anki_note);
    Ok((anki_note, anki_cards))
}

fn get_deck_by_name(collection: &AnkiCollection, deck_name: String) -> Result<&AnkiDeck> {
    collection
        .decks
        .iter()
        .find(|deck| deck.name == deck_name)
        .ok_or(anyhow!("Invalid name for deck: {}", deck_name))
}

fn get_model_by_name(collection: &AnkiCollection, model_name: String) -> Result<&AnkiModel> {
    collection
        .models
        .iter()
        .find(|model| model.name == model_name)
        .ok_or(anyhow!("Invalid name for model: {}", model_name))
}

fn get_new_anki_cards(model: &AnkiModel, deck: &AnkiDeck, anki_note: &AnkiNote) -> Vec<AnkiCard> {
    let unix_time = get_unix_time();
    model
        .tmpls
        .iter()
        .enumerate()
        .map(|(i, template)| AnkiCard {
            id: unix_time + (i as i64), // Make sure this is unique
            nid: anki_note.id,
            did: deck.id,
            ord: template.ord,
            mod_: get_unix_time(),
            usn: -1,           // force push
            type_: 0,          // new card
            queue: 0,          // new card
            due: anki_note.id, // Apparently I can use the note id here if the card is new.
            ivl: 0,            // This is the initial value I saw when adding a card in anki
            factor: 0,         // This is the initial value I saw when adding a card in anki
            reps: 0,
            lapses: 0,
            left: 0,
            odue: 0,
            odid: 0,
            flags: 0,
            data: "".to_string(),
        })
        .collect()
}

pub fn get_unix_time() -> i64 {
    let unix_time_u128: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get system time.")
        .as_millis();
    i64::try_from(unix_time_u128)
        .expect("Basically the year 2000 bug with unix time running over long. What year is it?")
}

fn verify_fields(fields: &mut Vec<String>, model: &AnkiModel) {
    // add_note_id_as_field(&mut fields, unix_time);
}

pub fn get_new_anki_note(mut fields: Vec<String>, model: &AnkiModel) -> AnkiNote {
    let unix_time = get_unix_time();
    verify_fields(&mut fields, model);
    let guid = unix_time; // I need a unique id here but I dont know how to do much better than just using the id
    let joined_fields = fields.join("");
    let csum = get_csum(&fields[0].clone());
    AnkiNote {
        id: unix_time,
        guid: guid.to_string(),
        mid: model.id,
        mod_: unix_time,
        usn: -1, // Force pushing to server
        tags: "".to_string(),
        flds: joined_fields,
        sfld: fields[0].clone(),
        csum,
        flags: 0,
        data: "".to_string(),
    }
}

pub fn add_anki_note(connection: &Connection, anki_note: AnkiNote) -> rusqlite::Result<usize> {
    connection.execute(
        "INSERT INTO notes (id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![anki_note.id, anki_note.guid, anki_note.mid, anki_note.mod_, anki_note.usn, anki_note.tags, anki_note.flds, anki_note.sfld, anki_note.csum, anki_note.flags, anki_note.data]
    )
}

pub fn add_anki_card(connection: &Connection, anki_card: AnkiCard) -> rusqlite::Result<usize> {
    connection.execute(
        "INSERT INTO cards (id, nid, did, ord, mod, usn, type, queue, due, ivl, factor, reps, lapses, left, odue, odid, flags, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        params![anki_card.id, anki_card.nid, anki_card.did, anki_card.ord, anki_card.mod_, anki_card.usn, anki_card.type_, anki_card.queue, anki_card.due, anki_card.ivl, anki_card.factor, anki_card.reps, anki_card.lapses, anki_card.left, anki_card.odue, anki_card.odid, anki_card.flags, anki_card.data]
    )
}

pub fn read_notes(connection: &Connection) -> rusqlite::Result<Vec<AnkiNote>> {
    let mut stmt = connection.prepare(
        "SELECT id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data FROM notes",
    )?;
    let note_iterator = stmt.query_map(params![], |row| {
        Ok(AnkiNote {
            id: row.get(0)?,
            guid: row.get(1)?,
            mid: row.get(2)?,
            mod_: row.get(3)?,
            usn: row.get(4)?,
            tags: row.get(5)?,
            flds: row.get(6)?,
            sfld: row.get(7)?,
            csum: row.get(8)?,
            flags: row.get(9)?,
            data: row.get(10)?,
        })
    })?;

    Ok(note_iterator
        .filter_map(|anki_note| anki_note.ok())
        .collect())
}

pub fn read_collection(connection: &Connection) -> rusqlite::Result<AnkiCollection> {
    let mut stmt = connection.prepare(
        "SELECT id, crt, mod, scm, ver, dty, usn, ls, conf, models, decks, dconf, tags FROM col",
    )?;
    let mut collection_iterator = stmt.query_map(params![], |row| {
        Ok(AnkiCollection {
            id: row.get(0)?,
            crt: row.get(1)?,
            mod_: row.get(2)?,
            scm: row.get(3)?,
            ver: row.get(4)?,
            dty: row.get(5)?,
            usn: row.get(6)?,
            ls: row.get(7)?,
            conf: row.get(8)?,
            models: get_anki_models_from_json(row.get(9)?).unwrap(),
            decks: get_anki_decks_from_json(row.get(10)?).unwrap(),
            dconf: row.get(11)?,
            tags: row.get(12)?,
        })
    })?;
    let collection = collection_iterator
        .next()
        .expect("No row in collection table");
    assert!(collection_iterator.next().is_none()); // We should only have one row in this table

    Ok(collection?)
}
