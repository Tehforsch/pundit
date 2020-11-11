use serde_derive::Deserialize;
use serde_json::{Result, Value};

#[derive(Debug, Deserialize)]
pub struct AnkiField {
    font: String,       // display font
    media: Vec<String>, // array of media. appears to be unused,
    name: String,       // field name,
    ord: i64,           // ordinal of the field - goes from 0 to num fields -1,
    rtl: bool,          // boolean, right-to-left script,
    size: i64,          // font size,
    sticky: bool,       // sticky fields retain the value that was last added when adding new notes
}

// Array of arrays describing, for each template T, which fields are required to generate T.
// The array is of the form [T,string,list], where:
// -  T is the ordinal of the template.
// - The string is 'none', 'all' or 'any'.
// - The list contains ordinal of fields, in increasing order.
// The meaning is as follows:
// - if the string is 'none', then no cards are generated for this template. The list should be empty.
// - if the string is 'all' then the card is generated only if each field of the list are filled
// - if the string is 'any', then the card is generated if any of the field of the list is filled.
//
// The algorithm to decide how to compute req from the template is explained on:
// https://github.com/Arthur-Milchior/anki/blob/commented/documentation//templates_generation_rules.md"
#[derive(Debug, Deserialize)]
pub struct AnkiFieldRequirement {
    pub ord_id: i64,
    pub type_: String,
    #[serde(rename(deserialize = "fieldOrdinals"))]
    pub field_ordinals: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AnkiCardTemplate {
    pub afmt: String,     // answer template string,
    pub bafmt: String,    // browser answer format: used for displaying answer in browser,
    pub bqfmt: String,    // browser question format: used for displaying question in browser,
    pub did: Option<i64>, // deck override (null by default),
    pub name: String,     // template name,
    pub ord: i64,         // template number, see flds,
    pub qfmt: String,     // question format string
}

#[derive(Debug, Deserialize)]
pub struct AnkiModel {
    pub css: String, // "CSS, shared for all templates"
    pub did: i64,    // Long specifying the id of the deck that cards are added to by default
    pub flds: Vec<AnkiField>,
    pub id: i64, // model ID, matches notes.mid,
    #[serde(rename(deserialize = "latexPost"))]
    pub latex_post: String, // String added to end of LaTeX expressions (usually \\end{document}),
    #[serde(rename(deserialize = "latexPre"))]
    pub latex_pre: String, // preamble for LaTeX expressions,
    #[serde(rename(deserialize = "mod"))]
    pub mod_: i64, // modification time in seconds,
    pub name: String, // model name,
    pub req: Vec<AnkiFieldRequirement>,
    pub sortf: i64, // Integer specifying which field is used for sorting in the browser,
    pub tags: Vec<String>, // Anki saves the tags of the last added note to the current model, use an empty array [],
    pub tmpls: Vec<AnkiCardTemplate>,
    #[serde(rename(deserialize = "type"))]
    pub type_: i64, // Integer specifying what type of model. 0 for standard, 1 for cloze,
    pub usn: i64, // usn: Update sequence number: used in same way as other usn vales in db,
    pub vers: Vec<String>, // Legacy version number (unused), use an empty array []
}

pub fn get_anki_models_from_json(json_data: String) -> Result<Vec<AnkiModel>> {
    let v: Value = serde_json::from_str(&json_data)?;
    if let Value::Object(coll) = v {
        Ok(coll
            .keys()
            .filter_map(|model_value| get_anki_model(&coll[model_value]).ok())
            .collect())
    } else {
        panic!("Invalid anki database: model json is not a map of id/model pairs.");
    }
}

pub fn get_anki_model(model_value: &Value) -> Result<AnkiModel> {
    serde_json::from_value::<AnkiModel>(model_value.clone())
}
