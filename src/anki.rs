use crypto::digest::Digest;
use crypto::sha1::Sha1;
use rusqlite::{params, Connection, Result};
use std::convert::TryFrom;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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

struct AnkiCard {
    id: i64, // integer primary key, the epoch milliseconds of when the card was created
    nid: i64, // integer not null, notes.id
    did: i64, // integer not null, deck id (available in col table)
    ord: i64, // integer not null, ordinal : identifies which of the card templates or cloze deletions it corresponds to - for card templates, valid values are from 0 to num templates - 1 - for cloze deletions, valid values are from 0 to max cloze index - 1 (they're 0 indexed despite the first being called `c1`)
    mod_: i64, // integer not null, modificaton time as epoch seconds
    usn: i64, // integer not null, update sequence number : used to figure out diffs when syncing.   value of -1 indicates changes that need to be pushed to server. usn < server usn indicates changes that need to be pulled from server.
    type_: i64, // integer not null, 0=new, 1=learning, 2=review, 3=relearning
    queue: i64, // integer not null, -3=user buried(In scheduler 2), -2=sched buried (In scheduler 2), -2=buried(In scheduler 1), -1=suspended, 0=new, 1=learning, 2=review (as for type) , 3=in learning, next rev in at least a day after the previous review , 4=preview
    due: i64, // integer not null, Due is used differently for different card types: new: note id or random int, due: integer day, relative to the collection's creation time, learning: integer timestamp
    ivl: i64, // integer not null, interval (used in SRS algorithm). Negative = seconds, positive = days
    factor: i64, // integer not null, The ease factor of the card in permille (parts per thousand). If the ease factor is 2500, the card’s interval will be multiplied by 2.5 the next time you press Good.
    reps: i64, // integer not null, number of reviews
    lapses: i64, // integer not null, the number of times the card went from a "was answered correctly", to "was answered incorrectly" state
    left: i64, // integer not null, of the form a*1000+b, with: b the number of reps left till graduation , a the number of reps left today
    odue: i64, // integer not null, original due: In filtered decks, it's the original due date that the card had before moving to filtered. If the card lapsed in scheduler1, then it's the value before the lapse. (This is used when switching to scheduler 2. At this time, cards in learning becomes due again, with their previous due date). In any other case it's 0.
    odid: i64, // integer not null, original did: only used when the card is currently in filtered deck flags           integer not null, an integer. This integer mod 8 represents a "flag", which can be see in browser and while reviewing a note. Red 1, Orange 2, Green 3, Blue 4, no flag: 0. This integer divided by 8 represents currently nothing
    data: String // text not null - currently unused
}

pub fn run_anki() -> Result<()> {
    let path = Path::new("../collection.anki2");
    let connection = Connection::open(path).unwrap();
    let _notes = read_database(&connection)?;
    let anki_note = get_new_anki_note(vec!["Heyoo".to_string(), "lolipop".to_string()]);
    add_anki_note(&connection, anki_note)?;
    connection.close()?;
    Ok(())
}

pub fn get_csum(input: &str) -> i64 {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    i64::from_str_radix(&hasher.result_str()[..8], 16).expect("Invalid hash result")
}

fn add_note_id_as_field(fields: &mut Vec<String>, note_id: i64) {
    fields.insert(0, note_id.to_string());
}

pub fn get_new_anki_note(mut fields: Vec<String>) -> AnkiNote {
    let unix_time_u128: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("How can this fail?")
        .as_millis();
    let unix_time: i64 = i64::try_from(unix_time_u128).expect("Yo its too late for this.");
    add_note_id_as_field(&mut fields, unix_time);
    let guid = unix_time; // I need a unique id here but I dont know how to do much better than just using the id
    let joined_fields = fields.join("");
    let csum = get_csum(&fields[0].clone());
    AnkiNote {
        id: unix_time,
        guid: guid.to_string(),
        mid: 1479919492012,
        mod_: unix_time,
        usn: -1, // Force pushing to server
        tags: "".to_string(),
        flds: joined_fields,
        sfld: fields[0].clone(),
        csum: csum,
        flags: 0,
        data: "".to_string(),
    }
}

pub fn add_anki_note(connection: &Connection, anki_note: AnkiNote) -> Result<()> {
    let res = connection.execute(
        "INSERT INTO notes (id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![anki_note.id, anki_note.guid, anki_note.mid, anki_note.mod_, anki_note.usn, anki_note.tags, anki_note.flds, anki_note.sfld, anki_note.csum, anki_note.flags, anki_note.data]
    );
    Ok(())
}


pub fn add_anki_card(connection: &Connection, anki_note: AnkiNote) -> Result<()> {
    let res = connection.execute(
        "INSERT INTO notes (id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![anki_note.id, anki_note.guid, anki_note.mid, anki_note.mod_, anki_note.usn, anki_note.tags, anki_note.flds, anki_note.sfld, anki_note.csum, anki_note.flags, anki_note.data]
    );
    Ok(())
}

pub fn read_database(connection: &Connection) -> Result<(Vec<AnkiNote>)> {
    let mut stmt = connection.prepare(
        "SELECT id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data FROM notes",
    )?;
    let person_iter = stmt.query_map(params![], |row| {
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

    Ok(person_iter.filter_map(|anki_note| anki_note.ok()).collect())
}
