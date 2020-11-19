#[derive(Debug)]
// See: https://github.com/ankidroid/Anki-Android/wiki/Database-Structure
pub struct AnkiNote {
    pub id: i64,      // integer primary key - epoch miliseconds of when the note was created
    pub guid: String, // text not null - globally unique id, almost certainly used for syncing
    pub mid: i64,     // integer not null - model id
    pub mod_: i64,    // integer not null - modification timestamp, epoch seconds
    pub usn: i64, // integer not null - update sequence number: for finding diffs when syncing. See the description in the cards table for more info
    pub tags: String, // text not null - space-separated string of tags. includes space at the beginning and end, for LIKE "% tag %" queries
    pub flds: String, // text not null - the values of the fields in this note. separated by 0x1f (31) character.
    pub sfld: String, // integer not null, - sort field: used for quick sorting and duplicate check. The sort field is an integer so that when users are sorting on a field that contains only numbers, they are sorted in numeric instead of lexical order. Text is stored in this integer field.
    pub csum: i64, // integer not null - field checksum used for duplicate check - integer representation of first 8 digits of sha1 hash of the first field
    pub flags: i64, // integer not null - unused
    pub data: String, // text not null - unused
}
