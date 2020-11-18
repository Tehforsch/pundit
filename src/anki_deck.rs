use serde_derive::Deserialize;
use serde_json::{Result, Value};

#[derive(Debug, Deserialize)]
pub struct AnkiDeck {
    pub name: String, // name of deck
    #[serde(rename(deserialize = "extendRev"))]
    pub extend_rev: Option<i64>, // extended review card limit (for custom study). Potentially absent, in this case it's considered to be 10 by aqt.customstudy ,
    pub usn: i64, // usn: Update sequence number: used in same way as other usn vales in db ,
    pub collapsed: bool, // true when deck is collapsed ,
    // pub browserCollapsed: Option<bool>, // true when deck collapsed in browser sometimes not available?
    // First one is the number of days that have passed between the collection was created and the deck was last updated. The second one is equal to the number of cards seen today in this deck minus the number of new cards in custom study today. BEWARE, it's changed in anki.sched(v2).Scheduler._updateStats and anki.sched(v2).Scheduler._updateCutoff.update  but can't be found by grepping 'newToday', because it's instead written as type+"Today" with type which may be new/rev/lrnToday
    #[serde(rename(deserialize = "newToday"))]
    pub new_today: [i64; 2],
    #[serde(rename(deserialize = "revToday"))]
    pub rev_today: [i64; 2],
    #[serde(rename(deserialize = "lrnToday"))]
    pub lrn_today: [i64; 2],
    #[serde(rename(deserialize = "timeToday"))]
    pub time_today: [i64; 2], // two number array used somehow for custom study. Currently unused in the code ,
    #[serde(rename(deserialize = "dyn"))]
    pub dyn_: i64, // "1 if dynamic (AKA filtered) deck",
    #[serde(rename(deserialize = "extendNew"))]
    pub extend_new: Option<i64>, // extended new card limit (for custom study). Potentially absent, in this case it's considered to be 10 by aqt.customstudy ,
    pub conf: Option<i64>, // id of option group from dconf in `col` table. Or absent if the deck is dynamic. Its absent in filtered deck,
    pub id: i64,           // deck ID (automatically generated long)
    #[serde(rename(deserialize = "mod"))]
    pub mod_: i64, // last modification time ,
    pub desc: String,      // deck description
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
