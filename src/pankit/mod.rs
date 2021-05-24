pub mod pankit_note_info;
pub mod pankit_yaml_block;
pub mod pankit_yaml_note;

use crate::anki::{find_anki_note_in_collection, named::get_by_name};
use crate::anki::get_csum;
use crate::anki::get_unix_time;
use crate::anki::is_note_id_field;
use crate::anki::update_anki_note_contents;
use crate::args::ConflictHandling;
use crate::config::ID_MULTIPLIER;
use crate::fzf::select_interactively;
use crate::note::Note;
use crate::notes::Notes;
use crate::{anki::anki_deck::AnkiDeck, config::ANKI_NOTE_FIELD_TEMPLATE};
use crate::{anki::anki_model::AnkiModel, config::ANKI_NOTE_TEMPLATE};
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use rusqlite::Connection;
use std::path::Path;
use std::{fs, path::PathBuf};

use std::cmp::Ordering::{Equal, Greater, Less};

use rand::Rng;

use self::pankit_note_info::PankitNoteInfo;
use self::{pankit_note_info::PankitDatabase, pankit_yaml_block::PankitYamlBlock};
use crate::anki::anki_card::AnkiCard;
use crate::anki::anki_collection::AnkiCollection;
use crate::anki::anki_note::AnkiNote;
use crate::anki::AnkiNoteInfo;
use crate::anki::{
    add_anki_card, add_anki_note, close_connection, get_new_anki_note_and_cards, read_collection,
    read_notes,
};

#[derive(Debug)]
enum Action<'a> {
    ChangeInDatabase(&'a AnkiNote),
    ChangeInDatabaseAndAnki(&'a AnkiNote),
    ChangeInDatabaseAndPundit(&'a AnkiNote),
    AddNoteAndCards(&'a AnkiNote, &'a [AnkiCard]),
    AskUserConflict(NoteConflict<'a>),
}

#[derive(Debug)]
struct NoteConflict<'a> {
    anki: &'a AnkiNote,
    pundit: &'a AnkiNote,
}

impl<'a> Action<'a> {
    pub fn is_conflict(&self) -> bool {
        matches!(self, Action::AskUserConflict(_))
    }
}

pub fn update_anki(
    path: &Path,
    pankit_db_path: &Path,
    notes: &Notes,
    conflict_handling: ConflictHandling,
) -> Result<()> {
    let mut pankit_db = read_pankit_database(pankit_db_path)?;
    let connection = Connection::open(path).unwrap();
    let anki_notes = read_notes(&connection)?;
    let collection = read_collection(&connection)?;
    update_from_pundit_contents(
        &connection,
        notes,
        &collection,
        &anki_notes,
        &mut pankit_db,
        conflict_handling,
    )?;
    close_connection(connection)?;
    write_pankit_database(pankit_db_path, &pankit_db)?;
    Ok(())
}

fn write_pankit_database(pankit_db_path: &Path, pankit_db: &PankitDatabase) -> Result<()> {
    let data = serde_yaml::to_string(pankit_db).context("While converting pankit db to yaml")?;
    fs::write(pankit_db_path, data).context("Unable to write pankit file")?;
    Ok(())
}

fn read_pankit_database(pankit_db_path: &Path) -> Result<PankitDatabase> {
    let mb_data = fs::read_to_string(pankit_db_path);
    match mb_data {
        Ok(data) => Ok(serde_yaml::from_str(&data).context("Reading pankit database contents")?),
        Err(_err) => {
            println!("Pankit database file does not exist: Assuming empty database.");
            Ok(PankitDatabase::new())
        }
    }
}

pub fn update_from_pundit_contents(
    connection: &Connection,
    notes: &Notes,
    collection: &AnkiCollection,
    anki_notes: &[AnkiNote],
    mut pankit_db: &mut PankitDatabase,
    conflict_handling: ConflictHandling,
) -> Result<()> {
    let anki_notes_and_cards = get_anki_notes_and_cards_for_pundit_notes(collection, notes)?;
    let actions: Vec<Action> = anki_notes_and_cards
        .iter()
        .map(|(anki_note, anki_cards)| get_action(pankit_db, anki_notes, anki_note, anki_cards))
        .collect();
    let filtered_actions = filter_actions_for_conflicts(actions, conflict_handling)?;
    for action in filtered_actions {
        execute_action(connection, &mut pankit_db, action)?;
    }
    Ok(())
}

fn filter_actions_for_conflicts(
    actions: Vec<Action>,
    conflict_handling: ConflictHandling,
) -> Result<Vec<Action>> {
    match conflict_handling {
        ConflictHandling::GiveError => get_actions_if_no_conflict(actions),
        ConflictHandling::Ignore => Ok(filter_conflict_actions(actions)),
        ConflictHandling::Anki => Ok(use_anki_in_case_of_conflicts(actions)),
        ConflictHandling::Pundit => Ok(use_pundit_in_case_of_conflicts(actions)),
    }
}

fn use_pundit_in_case_of_conflicts(actions: Vec<Action>) -> Vec<Action> {
    actions
        .into_iter()
        .map(|action| match action {
            Action::AskUserConflict(conflict) => Action::ChangeInDatabaseAndAnki(conflict.pundit),
            _ => action,
        })
        .collect()
}

fn use_anki_in_case_of_conflicts(actions: Vec<Action>) -> Vec<Action> {
    actions
        .into_iter()
        .map(|action| match action {
            Action::AskUserConflict(conflict) => Action::ChangeInDatabaseAndPundit(conflict.anki),
            _ => action,
        })
        .collect()
}

fn filter_conflict_actions(actions: Vec<Action>) -> Vec<Action> {
    actions
        .into_iter()
        .filter(|action| !action.is_conflict())
        .collect()
}

fn get_actions_if_no_conflict(actions: Vec<Action>) -> Result<Vec<Action>> {
    if actions.iter().any(|action| action.is_conflict()) {
        for conflict in actions.iter().filter(|action| action.is_conflict()) {
            println!("{:?}", conflict);
        }
        Err(anyhow!("There are conflicting notes!"))
    } else {
        Ok(actions)
    }
}

fn execute_action(
    connection: &Connection,
    pankit_db: &mut PankitDatabase,
    action: Action,
) -> Result<()> {
    match action {
        Action::AddNoteAndCards(anki_note, anki_cards) => {
            add_anki_note(&connection, anki_note)
                .context(format!("While adding anki note {}", anki_note.id))?;
            for anki_card in anki_cards {
                add_anki_card(&connection, anki_card).context("While adding anki card")?;
            }
        }
        Action::ChangeInDatabase(note) => {
            update_database_entry(pankit_db, note);
        }
        Action::ChangeInDatabaseAndAnki(note) => {
            update_anki_note_contents(connection, note)?;
            update_database_entry(pankit_db, note);
        }
        Action::AskUserConflict(conflict) => {
            println!(
                "There is a conflict between note contents for id {}:Pundit note contents:\n{}\nAnki note contents:\n{}",
                conflict.anki.id, conflict.anki.flds, conflict.pundit.flds
            );
        }
        Action::ChangeInDatabaseAndPundit(_note) => {
            todo!();
        }
    };
    Ok(())
}

fn get_action<'a>(
    pankit_db: &PankitDatabase,
    anki_notes: &'a [AnkiNote],
    anki_note: &'a AnkiNote,
    anki_cards: &'a [AnkiCard],
) -> Action<'a> {
    match find_anki_note_in_collection(anki_notes, anki_note) {
        None => Action::AddNoteAndCards(anki_note, anki_cards),
        Some(anki_note_in_collection) => {
            get_update_action(pankit_db, anki_note, anki_note_in_collection)
        }
    }
}

fn get_update_action<'a>(
    pankit_db: &PankitDatabase,
    anki_note_pundit: &'a AnkiNote,
    anki_note_anki: &'a AnkiNote,
) -> Action<'a> {
    let anki_csum = get_csum(&anki_note_anki.flds);
    let pundit_csum = get_csum(&anki_note_pundit.flds);
    if anki_csum == pundit_csum {
        // Everything up to date between anki and pundit - simply update the database.
        Action::ChangeInDatabase(anki_note_anki)
    } else {
        match pankit_db.get(&anki_note_pundit.id) {
            None => {
                // No entry in the pankit db but conflicting notes in anki and pundit: Ask the user
                Action::AskUserConflict(NoteConflict {
                    anki: anki_note_anki,
                    pundit: anki_note_pundit,
                })
            }
            Some(entry) => {
                if entry.csum == anki_csum {
                    // New contents in pundit that havent been introduced into anki / the pankit db yet
                    Action::ChangeInDatabaseAndAnki(anki_note_pundit)
                } else if entry.csum == pundit_csum {
                    // Anki contents differ from the pundit/pankit contents. This could be for one of two reasons:
                    // 1. The anki note was changed (most likely). In this case we want to pull the changes from anki
                    // 2. In a previous run, we succesfully updated the pankit database but failed to update the
                    // anki database for some reason. In this case we want to push the changes to anki
                    // To check which is the case, we compare the modification times in the pankit database and anki
                    // In the first case, the anki timestamp is later than the pankit timestamp
                    // In the second case, they are equal.
                    // If the anki timestamp is later than the pankit timestamp, something strange happened. We'll ask the user what to do
                    match anki_note_anki.mod_.cmp(&entry.mod_) {
                        Greater => Action::ChangeInDatabaseAndPundit(anki_note_anki),
                        Equal => Action::ChangeInDatabaseAndAnki(anki_note_pundit),
                        Less => Action::AskUserConflict(NoteConflict {
                            anki: anki_note_anki,
                            pundit: anki_note_pundit,
                        }),
                    }
                } else {
                    // All three checksums are different. Clearly a conflict, ask the user
                    Action::AskUserConflict(NoteConflict {
                        anki: anki_note_anki,
                        pundit: anki_note_pundit,
                    })
                }
            }
        }
    }
}

fn update_database_entry(pankit_db: &mut PankitDatabase, anki_note: &AnkiNote) {
    pankit_db.insert(
        anki_note.id,
        PankitNoteInfo {
            csum: get_csum(&anki_note.flds),
            mod_: anki_note.mod_,
        },
    );
}

pub fn get_anki_notes_and_cards_for_pundit_notes(
    collection: &AnkiCollection,
    notes: &Notes,
) -> Result<Vec<(AnkiNote, Vec<AnkiCard>)>> {
    let mut results = vec![];
    for pundit_note in notes.iter() {
        results.extend(get_anki_notes_and_cards_for_pundit_note(
            collection,
            pundit_note,
        )?)
    }
    Ok(results)
}

fn get_anki_notes_and_cards_for_pundit_note(
    collection: &AnkiCollection,
    pundit_note: &Note,
) -> Result<Vec<(AnkiNote, Vec<AnkiCard>)>> {
    get_anki_info_for_pundit_note(pundit_note)
        .context(format!(
            "While reading anki entries from note {}",
            pundit_note.title
        ))?
        .iter()
        .map(|anki_note_info| get_new_anki_note_and_cards(collection, anki_note_info))
        .collect::<Result<Vec<(AnkiNote, Vec<AnkiCard>)>>>()
        .context(format!(
            "While making new anki cards out of note {}",
            pundit_note.title
        ))
}

fn get_anki_info_for_pundit_note(pundit_note: &Note) -> Result<Vec<AnkiNoteInfo>> {
    let contents = pundit_note
        .get_contents()
        .context("While reading file contents")?;
    get_anki_info_from_note_contents(contents).context(format!(
        "Reading anki notes from file {:?}",
        pundit_note.filename
    ))
}

fn get_anki_info_from_note_contents(contents: String) -> Result<Vec<AnkiNoteInfo>> {
    let re = get_anki_block_regex();
    let mut result = vec![];
    for capture in re.captures_iter(&contents) {
        let data = capture.get(1).unwrap().as_str();
        let block_result: Result<PankitYamlBlock> = serde_yaml::from_str(&data).context(format!(
            "Reading note contents: {}",
            data
        ));
        result.extend(block_result?.into_notes()?);
    }
    Ok(result)
}

fn get_anki_block_regex() -> Regex {
    Regex::new(r"\#\+(?mis)begin_src *yaml *\n(.*)(?i)\#\+end_src").unwrap()
}

pub fn pankit_get_note(
    database: &std::path::PathBuf,
    model_filename: Option<PathBuf>,
) -> Result<()> {
    let connection = Connection::open(database).unwrap();
    let collection = read_collection(&connection)?;
    close_connection(connection)?;
    let id = get_new_note_id();
    let (model, deck) = get_model_and_deck_for_new_note(&collection, model_filename)?;
    print_anki_note(id, &model, &deck);
    Ok(())
}

fn get_model_and_deck_for_new_note(
    collection: &AnkiCollection,
    model_filename: Option<PathBuf>,
) -> Result<(&AnkiModel, &AnkiDeck)> {
    let maybe_model_and_deck_name =
        model_filename.and_then(|filename| get_model_and_deck_name_from_first_anki_note(filename).transpose()).transpose()?;
    Ok(match maybe_model_and_deck_name {
        Some((model_name, deck_name)) => (get_by_name(&collection.models, &model_name).unwrap(), get_by_name(&collection.decks, &deck_name).unwrap()),
        None => 
        (select_model_interactively(&collection).ok_or_else(|| anyhow!("No model selected"))?,
         select_deck_interactively(&collection).ok_or_else(|| anyhow!("No deck selected"))?)
    })
}

fn get_model_and_deck_name_from_first_anki_note(filename: PathBuf) -> Result<Option<(String, String)>> {
    let anki_notes = get_anki_info_from_note_contents(fs::read_to_string(&filename)?)?;
    Ok(anki_notes.first().map(|note| (note.model_name.clone(), note.deck_name.clone())))
}

fn print_anki_note(id: i64, model: &AnkiModel, deck: &AnkiDeck) {
    // I would use a simple yaml serialization here, but the problem is that
    // I explicitly want to keep the empty strings as empty space instead of
    // "" so instead I will just explicitly construct the template here.
    let fields_string = model
        .flds
        .iter()
        .map(|f| &f.name)
        .filter(|n| !is_note_id_field(n))
        .map(|field_name| {
            format!(
                "{}",
                ANKI_NOTE_FIELD_TEMPLATE.replace("{fieldName}", &field_name)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!(
        "{}",
        ANKI_NOTE_TEMPLATE
            .clone()
            .replace("{id}", &format!("{}", id))
            .replace("{model}", &model.name)
            .replace("{deck}", &deck.name)
            .replace("{fields}", &fields_string)
    );
}

fn get_new_note_id() -> i64 {
    let mut rng = rand::thread_rng();
    get_unix_time() * ID_MULTIPLIER + rng.gen_range(0, ID_MULTIPLIER)
}

fn select_model_interactively(collection: &AnkiCollection) -> Option<&AnkiModel> {
    select_interactively(&collection.models)
}

fn select_deck_interactively(collection: &AnkiCollection) -> Option<&AnkiDeck> {
    select_interactively(&collection.decks)
}
