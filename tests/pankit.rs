use std::path::Path;

use anyhow::{anyhow, Result};

pub mod setup;

use setup::{get_shell_command_output, run_setup_with_args, TestOutput};

pub static TEST_SETUPS_PATH: &str = "testSetupsPankit";
pub static DEFAULT_ANKI_SOURCE_COLLECTION_NAME: &str = "source.anki2";
pub static DEFAULT_ANKI_TARGET_COLLECTION_NAME: &str = "target.anki2";
pub static DEFAULT_PANKIT_FILE_NAME: &str = "pankit.yaml";

#[test]
fn test_add_existing_note_again() {
    let out = run_pankit_update_on_setup("addExistingNoteAgain", &[]).unwrap();
    assert!(out.success);
}

#[test]
fn test_add_note_to_empty_collection() {
    assert!(
        run_pankit_update_on_setup("addNoteToEmptyCollection", &[])
            .unwrap()
            .success
    );
}

#[test]
fn test_conflicting_note_contents_no_database() {
    let out = run_pankit_update_on_setup("conflictingNoteContentsNoDatabase", &[]).unwrap();
    assert!(!out.success); // The program should exit with an error because there is a conflict
}

#[test]
fn test_conflicting_note_contents_no_database_ignore() {
    let out = run_pankit_update_on_setup("conflictingNoteContentsNoDatabase", &["ignore"]).unwrap();
    assert!(out.success); // The program should simply ignore the conflict
}

#[test]
fn test_conflicting_note_contents_no_database_pundit() {
    let out =
        run_pankit_update_on_setup("conflictingNoteContentsNoDatabaseForcePundit", &["pundit"])
            .unwrap();
    assert!(out.success); // The program should use the changes from the pundit note and not give an error
}

#[test]
fn test_list_models() {
    let out = run_pankit_on_setup("pankit-list-models", "listModels", &[]).unwrap();
    assert!(out.output.contains("SomeModel"));
    assert!(out.output.contains("SomeModel2"));
    assert!(out.output.contains("SomeModel3"));
    assert!(out.output.contains("SomeModel4"));
    assert!(out.output.contains("SomeModel4"));
}

#[test]
fn test_list_decks() {
    let out = run_pankit_on_setup("pankit-list-decks", "listDecks", &[]).unwrap();
    assert!(out.output.contains("All"));
    assert!(out.output.contains("All::SubDeck"));
    assert!(out.output.contains("All::SubDeck::SubSubDeck"));
    assert!(out.output.contains("All::SubDeck2"));
    assert!(out.output.contains("SomeDeck"));
    assert!(out.output.contains("Default"));
}

fn get_sql_diff(database1: &Path, database2: &Path, tables: &[&str]) -> String {
    let mut args = vec![];
    for table in tables {
        args.extend_from_slice(&["--table", table]);
    }
    args.extend_from_slice(&[database1.to_str().unwrap(), database2.to_str().unwrap()]);
    let (_, output, _stderr) = get_shell_command_output("sqldiff", &args);
    output
}

/// Check that the two databases only differ in modification timestamps
fn check_same_notes_and_cards(database1: &Path, database2: &Path) -> Result<()> {
    let output = get_sql_diff(database1, database2, &["cards", "notes"]);
    println!("sqldiff output: {}", &output);
    for line in output.lines() {
        let mut words = line.split_whitespace();
        let instruction1 = words.next().ok_or(anyhow!("Not the same database"))?;
        let _database = words.next().ok_or(anyhow!("Not the same database"))?;
        let instruction2 = words.next().ok_or(anyhow!("Not the same database"))?;
        assert_eq!("UPDATE", instruction1);
        assert_eq!("SET", instruction2);
        loop {
            let next_word = words.next();
            if let Some(word) = next_word {
                // End of update statement
                if word == "WHERE" {
                    break;
                }
            }
            let mut key_value_split = next_word
                .ok_or(anyhow!("Not the same database"))?
                .split("=");
            let key = key_value_split
                .next()
                .ok_or(anyhow!("Not the same database"))?;
            let _value = key_value_split
                .next()
                .ok_or(anyhow!("Not the same database"))?;
            assert_eq!("mod", key); // Modification timestamp
        }
    }
    Ok(())
}

fn run_pankit_update_on_setup(setup_name: &str, args: &[&str]) -> Result<TestOutput> {
    let mut new_args = vec![DEFAULT_PANKIT_FILE_NAME];
    new_args.extend_from_slice(args);
    run_pankit_on_setup("pankit-update", setup_name, &new_args)
}

fn run_pankit_on_setup(command_name: &str, setup_name: &str, args: &[&str]) -> Result<TestOutput> {
    let mut new_args = vec![command_name, DEFAULT_ANKI_SOURCE_COLLECTION_NAME];
    new_args.extend_from_slice(args);
    let out = run_setup_with_args(Path::new(TEST_SETUPS_PATH), setup_name, &new_args);
    println!("STDOUT:\n{}", out.output);
    println!("STDERR:\n{}", out.stderr);
    check_same_notes_and_cards(
        &out.env.dir.path().join(DEFAULT_ANKI_SOURCE_COLLECTION_NAME),
        &out.env.dir.path().join(DEFAULT_ANKI_TARGET_COLLECTION_NAME),
    )?;
    Ok(out)
}
