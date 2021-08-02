use crate::anki::anki_deck::AnkiDeck;
use crate::anki::anki_model::AnkiModel;

#[derive(Debug)]
pub struct AnkiCollection {
    pub models: Vec<AnkiModel>,
    pub decks: Vec<AnkiDeck>,
}
