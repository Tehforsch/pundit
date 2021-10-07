use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

pub type PankitId = i64;

#[derive(Serialize, Deserialize)]
pub struct PankitNoteInfo {
    pub csum: i64,
    pub mod_: i64,
}

pub type PankitDatabase = HashMap<PankitId, PankitNoteInfo>;
