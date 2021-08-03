use crate::named::Named;
use crate::anki::proto::note_types::notetype::Config as NoteFieldConfig;
use rusqlite::{Connection, params};
use serde_derive::Deserialize;
use serde_json::{Result, Value};
use prost::Message;

use super::proto::note_types::{NotetypeId, notetype::Field as NoteField};

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
    let mut stmt = connection.prepare(
        "SELECT id, name, config FROM notetypes"
    )?;
    let mut models: Vec<AnkiModel> = stmt.query_map(params![], |row| {
        let config = NoteFieldConfig::decode(row.get_raw(2).as_blob()?).expect("Failed to decode anki model config");
        Ok(AnkiModel { 
            id: row.get(0)?,
            name: row.get(1)?,
            sortf: config.sort_field_idx as i64,
            tmpls: vec![],
            flds: vec![],
        })
    })?.collect::<rusqlite::Result<Vec<_>>>()?;
    for model in models.iter_mut() {
        model.flds = add_anki_fields_from_table(connection, model.id)?;
        // model.tmpls = add_anki_fields_from_table(connection, model.id)?;
    }
    Ok(models)
}

fn add_anki_fields_from_table(connection: &Connection, id: i64) -> rusqlite::Result<Vec<AnkiField>> {
    let mut stmt = connection.prepare("SELECT ord, name, config FROM FIELDS WHERE ntid = ? ORDER BY ord")?;
    let stmt_iterator = stmt.query_and_then([id], |row| {
        Ok(AnkiField {
            name: row.get(1)?,
        })
    })?;
    stmt_iterator.collect::<rusqlite::Result<Vec<_>>>()
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
