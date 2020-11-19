use crate::anki::anki_deck::AnkiDeck;
use crate::anki::anki_model::AnkiModel;

#[derive(Debug)]
pub struct AnkiCollection {
    pub id: i64,      // integer primary key arbitrary number since there is only one row
    pub crt: i64, // integer not null, timestamp of the creation date. It's correct up to the day. For V1 scheduler, // the hour corresponds to starting a new day. By default, // new day is 4.
    pub mod_: i64, // integer not null last modified in milliseconds
    pub scm: i64, // integer not null schema mod time: time when "schema" was modified. If server scm is different from the client scm a full-sync is required
    pub ver: i64, // integer not null, version
    pub dty: i64, // integer not null, dirty: unused, // set to 0
    pub usn: i64, // integer not null, update sequence number: used for finding diffs when syncing. See usn in cards table for more details.
    pub ls: i64,  // integer not null, last sync time
    pub conf: String, // text not null, json object containing configuration options that are synced
    pub models: Vec<AnkiModel>, // json array of json objects containing the models (aka Note types)
    pub decks: Vec<AnkiDeck>, // text not null, json array of json objects containing the deck
    pub dconf: String, // text not null, json array of json objects containing the deck options
    pub tags: String, // text not null, a cache of tags used in the collection (This list is displayed in the browser. Potentially at other place)
}
