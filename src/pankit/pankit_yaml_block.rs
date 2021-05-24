use std::{collections::{HashMap}};

use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

use crate::anki::AnkiNoteInfo;

use super::pankit_yaml_note::PankitYamlNote;

#[derive(Debug, Deserialize, Serialize)]
pub struct PankitYamlBlock {
    #[serde(flatten)]
    pub notes: HashMap<String, PankitYamlNote>,
    pub model: Option<String>,
    pub deck: Option<String>,
}

impl PankitYamlBlock {
    pub fn into_notes(self) -> Result<Vec<AnkiNoteInfo>> {
        let default_model = self.model.clone();
        let default_deck = self.deck.clone();
        self.notes.into_iter().map(|(id_string, note)| {
            Ok(AnkiNoteInfo {
                id: id_string.parse::<i64>()?,
                fields: note.fields,
                model_name: note.model.or(default_model.clone()).ok_or_else(|| anyhow!("No model specified for card: {}"))?,
                deck_name: note.deck.or(default_deck.clone()).ok_or_else(|| anyhow!("No deck specified for card: {}"))?,
            })
        }).collect()
    }
}
