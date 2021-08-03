use crate::named::Named;
use rusqlite::{Connection, params};
use serde_derive::Deserialize;
use serde_json::{Result, Value};

#[derive(Debug, Deserialize)]
pub struct AnkiDeck {
    pub name: String, // name of deck
    pub id: i64,           // deck ID (automatically generated long)
}

pub fn get_anki_decks_from_table(connection: &Connection) -> rusqlite::Result<Vec<AnkiDeck>> {
    let mut stmt = connection.prepare(
        "SELECT id, name FROM decks"
    )?;
    let iterator = stmt.query_map(params![], |row| {
        let name: String = row.get::<_, String>(1)?.replace("\u{1f}", "::");
        Ok(AnkiDeck {
            id: row.get(0)?,
            name,
        })
    })?;
    iterator.collect()
}

pub fn get_anki_decks_from_json(json_data: String) -> Result<Vec<AnkiDeck>> {
    let v: Value = serde_json::from_str(&json_data)?;
    if let Value::Object(coll) = v {
        coll.keys()
            .map(|deck_value| get_anki_deck(&coll[deck_value]))
            .collect()
    } else {
        panic!("Invalid anki database: deck json is not a map of id/deck pairs.");
    }
}

pub fn get_anki_deck(deck_value: &Value) -> Result<AnkiDeck> {
    serde_json::from_value::<AnkiDeck>(deck_value.clone())
}


impl Named for AnkiDeck {
    fn get_name(&self) -> &str {
        &self.name
    }
}
