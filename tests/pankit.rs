use std::path::Path;

use anyhow::Result;

mod setup;
mod sqlcheck;

use setup::{get_pundit_executable, run_pundit_on_setup_with_args, TestOutput};

use crate::sqlcheck::check_same_notes_and_cards;

pub static TEST_SETUPS_PATH: &str = "testSetupsPankit";
pub static DEFAULT_ANKI_SOURCE_COLLECTION_NAME: &str = "source.anki2";
pub static DEFAULT_ANKI_TARGET_COLLECTION_NAME: &str = "target.anki2";
pub static DEFAULT_PANKIT_FILE_NAME: &str = "pankit.yaml";

#[test]
fn test_add_existing_note_again() {
    let out = run_pankit_on_setup("addExistingNoteAgain", &[]).unwrap();
    assert!(out.success);
}

#[test]
fn test_add_note_to_empty_collection() {
    assert!(
        run_pankit_on_setup("addNoteToEmptyCollection", &[])
            .unwrap()
            .success
    );
}

#[test]
fn test_conflicting_note_contents_no_database() {
    let out = run_pankit_on_setup("conflictingNoteContentsNoDatabase", &[]).unwrap();
    assert!(!out.success); // The program should exit with an error because there is a conflict
}

#[test]
fn test_conflicting_note_contents_no_database_ignore() {
    let out = run_pankit_on_setup("conflictingNoteContentsNoDatabase", &["ignore"]).unwrap();
    assert!(out.success); // The program should simply ignore the conflict
}

#[test]
fn test_conflicting_note_contents_no_database_pundit() {
    let out =
        run_pankit_on_setup("conflictingNoteContentsNoDatabaseForcePundit", &["pundit"]).unwrap();
    assert!(out.success); // The program should use the changes from the pundit note and not give an error
}

fn run_pankit_on_setup(setup_name: &str, args: &[&str]) -> Result<TestOutput> {
    let mut new_args = vec![
        "pankit",
        DEFAULT_ANKI_SOURCE_COLLECTION_NAME,
        DEFAULT_PANKIT_FILE_NAME,
    ];
    new_args.extend_from_slice(args);
    let out = run_pundit_on_setup_with_args(
        get_pundit_executable(),
        Path::new(TEST_SETUPS_PATH),
        setup_name,
        &new_args,
    );
    println!("STDOUT:\n{}", out.output);
    println!("STDERR:\n{}", out.stderr);
    check_same_notes_and_cards(
        &out.env.dir.path().join(DEFAULT_ANKI_SOURCE_COLLECTION_NAME),
        &out.env.dir.path().join(DEFAULT_ANKI_TARGET_COLLECTION_NAME),
    )?;
    Ok(out)
}
