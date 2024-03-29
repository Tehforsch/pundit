pub mod anki_args;
pub mod anki_card;
pub mod anki_collection;
pub mod anki_deck;
pub mod anki_model;
pub mod anki_note;
pub mod proto;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use log::info;
use rusqlite::params;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

use self::anki_card::AnkiCard;
use self::anki_collection::AnkiCollection;
use self::anki_deck::get_anki_decks_from_json;
use self::anki_deck::AnkiDeck;
use self::anki_model::get_anki_models_from_json;
use self::anki_model::get_anki_models_from_table;
use self::anki_model::AnkiModel;
use self::anki_note::AnkiNote;
use crate::anki::anki_deck::get_anki_decks_from_table;
use crate::named::get_by_name;

#[derive(Debug)]
pub struct AnkiNoteInfo {
    pub id: i64,
    pub fields: HashMap<String, String>,
    pub model_name: String,
    pub deck_name: String,
}

pub struct AnkiBlock {
    pub model_name: Option<String>,
    pub deck_name: Option<String>,
    pub notes: Vec<AnkiNoteInfo>,
}

pub fn find_anki_note_in_collection<'a>(
    anki_notes: &'a [AnkiNote],
    note: &'a AnkiNote,
) -> Option<&'a AnkiNote> {
    anki_notes.iter().find(|n| n.id == note.id)
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

pub fn get_new_anki_note_and_cards(
    collection: &AnkiCollection,
    note_info: &AnkiNoteInfo,
) -> Result<(AnkiNote, Vec<AnkiCard>)> {
    let model = get_model_by_name(collection, &note_info.model_name)?;
    let deck = get_deck_by_name(collection, &note_info.deck_name)?;
    let anki_note = get_new_anki_note(note_info, model)?;
    let anki_cards = get_new_anki_cards(model, deck, &anki_note);
    Ok((anki_note, anki_cards))
}

pub fn get_deck_by_name<'a>(
    collection: &'a AnkiCollection,
    deck_name: &'a str,
) -> Result<&'a AnkiDeck> {
    get_by_name(&collection.decks, deck_name)
        .ok_or_else(|| anyhow!("Invalid name for deck: {}", deck_name))
}

pub fn get_model_by_name<'a>(
    collection: &'a AnkiCollection,
    model_name: &'a str,
) -> Result<&'a AnkiModel> {
    get_by_name(&collection.models, model_name)
        .ok_or_else(|| anyhow!("Invalid name for model: {}", model_name))
}

fn get_new_anki_cards(model: &AnkiModel, deck: &AnkiDeck, anki_note: &AnkiNote) -> Vec<AnkiCard> {
    model
        .tmpls
        .iter()
        .enumerate()
        .map(|(i, template)| AnkiCard {
            id: anki_note.id * 200 + (i as i64), // Make sure this is unique. Some decks have subsequent ids for their notes in which case we cannot simply add 1/2/3 to the note id to get the card id because we might get overlap. Multiplying by 200 ensures that there is enough space for 200 different cards without overlap.
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

pub fn is_note_id_field(field_name: &str) -> bool {
    field_name.to_lowercase() == "note id"
}

fn verify_fields(fields: &HashMap<String, String>, model: &AnkiModel) -> Result<bool> {
    let has_note_id_field = model.flds.iter().any(|field| is_note_id_field(&field.name));
    if has_note_id_field && !is_note_id_field(&model.flds[0].name) {
        return Err(anyhow!("Note ID field is not the first field."));
    }
    if fields.len() == model.flds.len() {
        match has_note_id_field {
            true => Err(anyhow!("Number of fields equal to number of fields in model but a note_id field is present.")),
            false => Ok(false)
        }
    } else if fields.len() == model.flds.len() - 1 {
        match has_note_id_field {
            true => {
                Ok(true)
            },
            false => Err(anyhow!("Number of fields 1 smaller than number of fields in model but a note_id field is not present."))
        }
    } else {
        Err(anyhow!(
            "Mismatch in number of fields given to number of fields in model: {} vs {}",
            fields.len(),
            model.flds.len()
        ))
    }
}

pub fn sort_fields_by_occurence(
    note_info: &AnkiNoteInfo,
    model: &AnkiModel,
) -> Result<Vec<String>> {
    model
        .flds
        .iter()
        .map(|fld| {
            if is_note_id_field(&fld.name) {
                Ok(note_info.id.to_string())
            } else {
                note_info
                    .fields
                    .get(&fld.name)
                    .ok_or_else(|| anyhow!("Entry for field not found: {}", fld.name))
                    .map(|x| x.to_string())
            }
        })
        .collect()
}

pub fn get_new_anki_note(note_info: &AnkiNoteInfo, model: &AnkiModel) -> Result<AnkiNote> {
    let unix_time = get_unix_time();
    let guid = note_info.id;
    verify_fields(&note_info.fields, model)
        .context(format!("While reading fields of note {}", note_info.id))?;
    let sorted_field_entries = sort_fields_by_occurence(note_info, model).context(format!(
        "Looking up field entries for note {}",
        note_info.id
    ))?;
    let separator = "";
    let joined_fields = sorted_field_entries.join(separator);
    let csum = get_csum(&sorted_field_entries[0].clone());
    let sort_field_name: String = model
        .flds
        .get(model.sortf as usize)
        .ok_or_else(|| anyhow!("Sort field entry in anki model is invalid for this model?"))?
        .name
        .to_string();
    let sfld_contents = get_sort_field_contents(&sort_field_name, &model, note_info)?;
    Ok(AnkiNote {
        id: note_info.id,
        guid: guid.to_string(),
        mid: model.id,
        mod_: unix_time,
        usn: -1, // Force pushing to server
        tags: "".to_string(),
        flds: joined_fields,
        sfld: sfld_contents,
        csum,
        flags: 0,
        data: "".to_string(),
    })
}

pub fn get_sort_field_contents(
    sort_field_name: &str,
    model: &AnkiModel,
    note_info: &AnkiNoteInfo,
) -> Result<String> {
    Ok(match is_note_id_field(&sort_field_name) {
        true => note_info.id.to_string(),
        false => note_info
            .fields
            .get(sort_field_name)
            .ok_or_else(|| {
                anyhow!(format!(
                    "Sort field entry refers to a field that does not exist: {} in model {}",
                    &sort_field_name, model.name,
                ))
            })?
            .clone(),
    })
}

pub fn add_anki_note(connection: &Connection, anki_note: &AnkiNote) -> rusqlite::Result<usize> {
    info!("Adding {}", anki_note.id);
    connection.execute(
        "INSERT INTO notes (id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![anki_note.id, anki_note.guid, anki_note.mid, anki_note.mod_, anki_note.usn, anki_note.tags, anki_note.flds, anki_note.sfld, anki_note.csum, anki_note.flags, anki_note.data]
    )
}

pub fn add_anki_card(connection: &Connection, anki_card: &AnkiCard) -> rusqlite::Result<usize> {
    info!("Adding card {}", anki_card.id);
    connection.execute(
        "INSERT INTO cards (id, nid, did, ord, mod, usn, type, queue, due, ivl, factor, reps, lapses, left, odue, odid, flags, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        params![anki_card.id, anki_card.nid, anki_card.did, anki_card.ord, anki_card.mod_, anki_card.usn, anki_card.type_, anki_card.queue, anki_card.due, anki_card.ivl, anki_card.factor, anki_card.reps, anki_card.lapses, anki_card.left, anki_card.odue, anki_card.odid, anki_card.flags, anki_card.data]
    )
}

pub fn update_anki_note_contents(
    connection: &Connection,
    anki_note: &AnkiNote,
) -> rusqlite::Result<usize> {
    info!("Updating note contents {}", anki_note.id);
    connection.execute(
        "UPDATE notes SET (mod, flds, sfld, csum) = (?1, ?2, ?3, ?4) WHERE id = (?5)",
        params![
            anki_note.mod_,
            anki_note.flds,
            anki_note.sfld,
            anki_note.csum,
            anki_note.id
        ],
    )
}

pub fn read_notes(connection: &Connection) -> Result<Vec<AnkiNote>> {
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
            sfld: get_string_from_sfld_row(row, 7).unwrap(),
            csum: row.get(8)?,
            flags: row.get(9)?,
            data: row.get(10)?,
        })
    })?;

    note_iterator
        .map(|anki_note| anki_note.context("Reading row in notes table"))
        .collect()
}

pub fn get_string_from_sfld_row(row: &rusqlite::Row, index: usize) -> Result<String> {
    // Sometimes the entry in sfld is an integer at which point rusqlite may think that it is indeed an
    // integer so that it fails to parse for the sfld entry (which most of the time is a string)
    // Force conversion to string here.
    let res = row.get_raw_checked(index)?;
    match res.data_type() {
        rusqlite::types::Type::Text => Ok(res.as_str()?.to_owned()),
        rusqlite::types::Type::Integer => Ok(res.as_i64()?.to_string().to_owned()),
        _ => Err(anyhow!("Invalid type in sfld entry!")),
    }
}

pub fn read_collection(connection: &Connection) -> rusqlite::Result<AnkiCollection> {
    let version = get_database_schema_version(connection)?;
    let collection = match version {
        11 | 14 => read_collection_version_14(connection),
        16 => read_collection_version_16(connection),
        _ => panic!("Unsupported database version: {}", version),
    };

    collection
}

fn read_collection_version_14(connection: &Connection) -> rusqlite::Result<AnkiCollection> {
    let mut stmt = connection.prepare("SELECT models, decks FROM col")?;
    let mut collection_iterator = stmt.query_map(params![], |row| {
        Ok(AnkiCollection {
            models: get_anki_models_from_json(row.get(0)?)
                .expect("Failed to read anki models json"),
            decks: get_anki_decks_from_json(row.get(1)?).expect("Failed to read anki decks json"),
        })
    })?;
    let collection = collection_iterator
        .next()
        .expect("No row in collection table");
    assert!(collection_iterator.next().is_none()); // We should only have one row in this table
    collection
}

fn read_collection_version_16(connection: &Connection) -> rusqlite::Result<AnkiCollection> {
    Ok(AnkiCollection {
        decks: get_anki_decks_from_table(connection).expect("Failed to read anki decks from table"),
        models: get_anki_models_from_table(connection)
            .expect("Failed to read anki models from table"),
    })
}

fn get_database_schema_version(connection: &Connection) -> rusqlite::Result<u8> {
    Ok(connection.query_row("select ver from col", NO_PARAMS, |r| Ok(r.get(0)?))?)
}
