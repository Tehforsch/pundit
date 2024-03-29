use std::path::Path;

use anyhow::Result;

mod setup;
mod sqlcheck;

use setup::get_pundit_executable;
use setup::run_pundit_on_setup_with_args;
use setup::show_output;
use setup::TestArg;
use setup::TestArg::NormalArg;
use setup::TestArg::RelativePath;
use setup::TestOutput;

use crate::sqlcheck::check_same_notes_and_cards;

pub static TEST_SETUPS_PATH: &str = "testSetupsPankit";
pub static DEFAULT_ANKI_SOURCE_COLLECTION_NAME: &str = "source.anki2";
pub static DEFAULT_ANKI_TARGET_COLLECTION_NAME: &str = "target.anki2";
pub static DEFAULT_PANKIT_FILE_NAME: &str = "pankit.yaml";

#[test]
fn add_existing_note_again() {
    let out = run_pankit_on_setup("addExistingNoteAgain", &[]).unwrap();
    assert!(out.success);
}

#[test]
fn add_note_to_empty_collection() {
    assert!(
        run_pankit_on_setup("addNoteToEmptyCollection", &[])
            .unwrap()
            .success
    );
}

#[test]
fn colons_in_deck_name() {
    assert!(
        run_pankit_on_setup("colonsInDeckName", &[])
            .unwrap()
            .success
    );
}

#[test]
fn spaces_in_deck_name() {
    assert!(
        run_pankit_on_setup("spacesInDeckName", &[])
            .unwrap()
            .success
    );
}

#[test]
fn conflicting_note_contents_no_database() {
    let out = run_pankit_on_setup("conflictingNoteContentsNoDatabase", &[]).unwrap();
    assert!(!out.success); // The program should exit with an error because there is a conflict
}

#[test]
fn conflicting_note_contents_no_database_ignore() {
    let out =
        run_pankit_on_setup("conflictingNoteContentsNoDatabase", &[NormalArg("ignore")]).unwrap();
    assert!(out.success); // The program should simply ignore the conflict
}

#[test]
fn conflicting_note_contents_no_database_pundit() {
    let out = run_pankit_on_setup(
        "conflictingNoteContentsNoDatabaseForcePundit",
        &[NormalArg("pundit")],
    )
    .unwrap();
    assert!(out.success); // The program should use the changes from the pundit note and not give an error
}

#[test]
fn add_note_default_deck_model() {
    assert!(
        run_pankit_on_setup("addNoteDefaultDeckModel", &[])
            .unwrap()
            .success
    );
}

#[test]
fn add_note_with_model_with_note_id_as_sort_field() {
    assert!(
        run_pankit_on_setup("addNoteWithModelWithNoteIdAsSortField", &[])
            .unwrap()
            .success
    );
}

#[test]
fn integer_in_sort_field() {
    assert!(
        run_pankit_on_setup("integerInSortField", &[])
            .unwrap()
            .success
    );
}

fn run_pankit_on_setup(setup_name: &str, args: &[TestArg]) -> Result<TestOutput> {
    let mut new_args = vec![
        NormalArg("pankit"),
        RelativePath(&DEFAULT_ANKI_SOURCE_COLLECTION_NAME),
        RelativePath(&DEFAULT_PANKIT_FILE_NAME),
    ];
    new_args.extend_from_slice(args);
    let out = run_pundit_on_setup_with_args(
        get_pundit_executable(),
        Path::new(TEST_SETUPS_PATH),
        setup_name,
        &new_args,
    )
    .unwrap();
    show_output(&out);
    check_same_notes_and_cards(
        &out.env.dir.path().join(DEFAULT_ANKI_SOURCE_COLLECTION_NAME),
        &out.env.dir.path().join(DEFAULT_ANKI_TARGET_COLLECTION_NAME),
    )?;
    Ok(out)
}
