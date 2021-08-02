use crate::named::Named;
use rusqlite::{Connection, params};
use serde_derive::Deserialize;
use serde_json::{Result, Value};

#[derive(Debug, Deserialize)]
pub struct AnkiField {
    pub name: String,               // field name,
}

#[derive(Debug, Deserialize)]
pub struct AnkiCardTemplate {
    pub ord: i64,         // template number, see flds,
}

#[derive(Debug, Deserialize)]
pub struct AnkiModel {
    pub flds: Vec<AnkiField>,
    pub id: i64, // model ID, matches notes.mid,
    pub name: String, // model name,
    pub sortf: i64, // Integer specifying which field is used for sorting in the browser,
    pub tmpls: Vec<AnkiCardTemplate>,
}

pub fn get_anki_models_from_table(connection: &Connection) -> rusqlite::Result<Vec<AnkiModel>> {
    todo!()
    // let mut stmt = connection.prepare(
    //     "SELECT id, name FROM decks"
    // )?;
    // stmt.query_map(params![], |row| {
    //     Ok(AnkiDeck {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //     })
    // }).collect()
}

pub fn get_anki_models_from_json(json_data: String) -> Result<Vec<AnkiModel>> {
    let v: Value = serde_json::from_str(&json_data)?;
    if let Value::Object(coll) = v {
        coll.keys()
            .map(|model_value| get_anki_model(&coll[model_value]))
            .collect()
    } else {
        panic!("Invalid anki database: model json is not a map of id/model pairs.");
    }
}

pub fn get_anki_model(model_value: &Value) -> Result<AnkiModel> {
    serde_json::from_value::<AnkiModel>(model_value.clone())
}


impl Named for AnkiModel {
    fn get_name(&self) -> &str {
        &self.name
    }
}
