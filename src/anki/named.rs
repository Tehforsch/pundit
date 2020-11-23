use super::anki_deck::AnkiDeck;
use super::anki_model::AnkiModel;

pub trait Named {
    fn get_name(&self) -> &str;
}

pub fn get_by_name<'a, T: Named>(items: &'a [T], name: &str) -> Option<&'a T> {
    items.iter().find(|item| item.get_name() == name)
}

impl Named for AnkiModel {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Named for AnkiDeck {
    fn get_name(&self) -> &str {
        &self.name
    }
}
