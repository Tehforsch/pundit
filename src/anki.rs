use crypto::digest::Digest;
use crypto::sha1::Sha1;
use rusqlite::{params, Connection};
use std::convert::TryFrom;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::anki_deck::{get_anki_decks_from_json, AnkiDeck};
use crate::anki_model::{get_anki_models_from_json, AnkiModel};
use crate::errors::InvalidNameError;

#[derive(Debug)]
// See: https://github.com/ankidroid/Anki-Android/wiki/Database-Structure
pub struct AnkiNote {
    id: i64,      // integer primary key - epoch miliseconds of when the note was created
    guid: String, // text not null - globally unique id, almost certainly used for syncing
    mid: i64,     // integer not null - model id
    mod_: i64,    // integer not null - modification timestamp, epoch seconds
    usn: i64, // integer not null - update sequence number: for finding diffs when syncing. See the description in the cards table for more info
    tags: String, // text not null - space-separated string of tags. includes space at the beginning and end, for LIKE "% tag %" queries
    flds: String, // text not null - the values of the fields in this note. separated by 0x1f (31) character.
    sfld: String, // integer not null, - sort field: used for quick sorting and duplicate check. The sort field is an integer so that when users are sorting on a field that contains only numbers, they are sorted in numeric instead of lexical order. Text is stored in this integer field.
    csum: i64, // integer not null - field checksum used for duplicate check - integer representation of first 8 digits of sha1 hash of the first field
    flags: i64, // integer not null - unused
    data: String, // text not null - unused
}

pub struct AnkiCard {
    id: i64,      // integer primary key, the epoch milliseconds of when the card was created
    nid: i64,     // integer not null, notes.id
    did: i64,     // integer not null, deck id (available in col table)
    ord: i64, // integer not null, ordinal : identifies which of the card templates or cloze deletions it corresponds to - for card templates, valid values are from 0 to num templates - 1 - for cloze deletions, valid values are from 0 to max cloze index - 1 (they're 0 indexed despite the first being called `c1`)
    mod_: i64, // integer not null, modificaton time as epoch seconds
    usn: i64, // integer not null, update sequence number : used to figure out diffs when syncing.   value of -1 indicates changes that need to be pushed to server. usn < server usn indicates changes that need to be pulled from server.
    type_: i64, // integer not null, 0=new, 1=learning, 2=review, 3=relearning
    queue: i64, // integer not null, -3=user buried(In scheduler 2), -2=sched buried (In scheduler 2), -2=buried(In scheduler 1), -1=suspended, 0=new, 1=learning, 2=review (as for type) , 3=in learning, next rev in at least a day after the previous review , 4=preview
    due: i64, // integer not null, Due is used differently for different card types: new: note id or random int, due: integer day, relative to the collection's creation time, learning: integer timestamp
    ivl: i64, // integer not null, interval (used in SRS algorithm). Negative = seconds, positive = days
    factor: i64, // integer not null, The ease factor of the card in permille (parts per thousand). If the ease factor is 2500, the card’s interval will be multiplied by 2.5 the next time you press Good.
    reps: i64,   // integer not null, number of reviews
    lapses: i64, // integer not null, the number of times the card went from a "was answered correctly", to "was answered incorrectly" state
    left: i64, // integer not null, of the form a*1000+b, with: b the number of reps left till graduation , a the number of reps left today
    odue: i64, // integer not null, original due: In filtered decks, it's the original due date that the card had before moving to filtered. If the card lapsed in scheduler1, then it's the value before the lapse. (This is used when switching to scheduler 2. At this time, cards in learning becomes due again, with their previous due date). In any other case it's 0.
    odid: i64, // integer not null, original did: only used when the card is currently in filtered deck
    flags: i64, // integer not null, an integer. This integer mod 8 represents a "flag", which can be see in browser and while reviewing a note. Red 1, Orange 2, Green 3, Blue 4, no flag: 0. This integer divided by 8 represents currently nothing
    data: String, // text not null - currently unused
}

#[derive(Debug)]
pub struct AnkiCollection {
    id: i64,                // integer primary key arbitrary number since there is only one row
    crt: i64, // integer not null, timestamp of the creation date. It's correct up to the day. For V1 scheduler, // the hour corresponds to starting a new day. By default, // new day is 4.
    mod_: i64, // integer not null last modified in milliseconds
    scm: i64, // integer not null schema mod time: time when "schema" was modified. If server scm is different from the client scm a full-sync is required
    ver: i64, // integer not null, version
    dty: i64, // integer not null, dirty: unused, // set to 0
    usn: i64, // integer not null, update sequence number: used for finding diffs when syncing. See usn in cards table for more details.
    ls: i64,  // integer not null, last sync time
    conf: String, // text not null, json object containing configuration options that are synced
    models: Vec<AnkiModel>, // json array of json objects containing the models (aka Note types)
    decks: Vec<AnkiDeck>, // text not null, json array of json objects containing the deck
    dconf: String, // text not null, json array of json objects containing the deck options
    tags: String, // text not null, a cache of tags used in the collection (This list is displayed in the browser. Potentially at other place)
}

pub fn run_anki() -> rusqlite::Result<()> {
    let path = Path::new("../collection.anki2");
    let connection = Connection::open(path).unwrap();
    let _notes = read_notes(&connection)?;
    let collection = read_collection(&connection)?;
    let arbitrary_model_name = "MachineLearning"; // TODO
    let arbitrary_deck_name = "All::ComputerScience"; // TODO
    let (anki_note, anki_cards) = get_new_anki_note_and_cards(
        &collection,
        vec!["A".to_string(), "B".to_string()],
        arbitrary_model_name.to_string(),
        arbitrary_deck_name.to_string(),
    )
    .expect("Name does not exist.");
    let num_added = add_anki_note(&connection, anki_note)?;
    println!("Added {} notes.", num_added);
    for anki_card in anki_cards {
        let num_cards_added = add_anki_card(&connection, anki_card)?;
        println!("Added {} cards.", num_cards_added);
    }
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
) -> Result<(AnkiNote, Vec<AnkiCard>), InvalidNameError> {
    let model = get_model_by_name(collection, model_name)?;
    let deck = get_deck_by_name(collection, deck_name)?;
    let anki_note = get_new_anki_note(fields, model.id);
    let anki_cards = get_new_anki_cards(model, deck, &anki_note);
    Ok((anki_note, anki_cards))
}

fn get_deck_by_name(
    collection: &AnkiCollection,
    deck_name: String,
) -> Result<&AnkiDeck, InvalidNameError> {
    collection
        .decks
        .iter()
        .find(|deck| deck.name == deck_name)
        .ok_or(InvalidNameError { name: deck_name })
}

fn get_model_by_name(
    collection: &AnkiCollection,
    model_name: String,
) -> Result<&AnkiModel, InvalidNameError> {
    collection
        .models
        .iter()
        .find(|model| model.name == model_name)
        .ok_or(InvalidNameError { name: model_name })
}

fn get_new_anki_cards(model: &AnkiModel, deck: &AnkiDeck, anki_note: &AnkiNote) -> Vec<AnkiCard> {
    let unix_time = get_unix_time();
    model
        .tmpls
        .iter()
        .map(|template| AnkiCard {
            id: unix_time, // Not super happy with this - what if we create those cards less than 1 ms apart?! But this is the system so I'm not sure what to do here.
            nid: anki_note.id,
            did: deck.id,
            ord: template.ord,
            mod_: unix_time,
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

pub fn get_new_anki_note(mut fields: Vec<String>, model_id: i64) -> AnkiNote {
    let unix_time = get_unix_time();
    add_note_id_as_field(&mut fields, unix_time);
    let guid = unix_time; // I need a unique id here but I dont know how to do much better than just using the id
    let joined_fields = fields.join("");
    let csum = get_csum(&fields[0].clone());
    AnkiNote {
        id: unix_time,
        guid: guid.to_string(),
        mid: model_id,
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
