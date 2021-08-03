use std::path::Path;

use anyhow::Result;
use setup::get_shell_command_output;
use setup::setup_test;

mod setup;
mod sqlcheck;

use crate::sqlcheck::check_same_notes_and_cards;
use setup::get_ankitool_executable;
use setup::TestOutput;

pub static TEST_SETUPS_PATH: &str = "testSetupsAnkitool";
pub static DEFAULT_ANKI_SOURCE_COLLECTION_NAME: &str = "source.anki2";
pub static DEFAULT_ANKI_TARGET_COLLECTION_NAME: &str = "target.anki2";
pub static DEFAULT_PANKIT_FILE_NAME: &str = "pankit.yaml";

#[test]
fn list_models() {
    let out = run_ankitool_on_setup("listModels", &["list-models"]).unwrap();
    assert!(out.output.contains("SomeModel"));
    assert!(out.output.contains("SomeModel2"));
    assert!(out.output.contains("SomeModel3"));
    assert!(out.output.contains("SomeModel4"));
    assert!(out.output.contains("SomeModel4"));
}

#[test]
fn list_decks() {
    let out = run_ankitool_on_setup("listDecks", &["list-decks"]).unwrap();
    assert!(out.output.contains("All"));
    assert!(out.output.contains("All::SubDeck"));
    assert!(out.output.contains("All::SubDeck::SubSubDeck"));
    assert!(out.output.contains("All::SubDeck2"));
    assert!(out.output.contains("SomeDeck"));
    assert!(out.output.contains("Default"));
}

#[test]
fn list_fields() {
    let out = run_ankitool_on_setup("listFields", &["list-fields", "SomeModel"]).unwrap();
    assert!(out.output.contains("Front"));
    assert!(out.output.contains("Back"));
    assert!(out.output.contains("SomeField1"));
    assert!(out.output.contains("SomeField2"));
    assert!(out.output.contains("SomeField3"));
}

#[test]
fn list_templates() {
    let out = run_ankitool_on_setup("listFields", &["list-templates", "SomeModel"]).unwrap();
    assert!(out.output.contains("Card 1"));
    assert!(out.output.contains("Card 2"));
}

#[test]
fn new_database_schema_list_decks() {
    let out = run_ankitool_on_setup("newDatabaseSchemaListDecks", &["list-decks"]).unwrap();
    assert!(out.output.contains("All"));
    assert!(out.output.contains("All::SubDeck"));
    assert!(out.output.contains("All::SubDeck::SubSubDeck"));
    assert!(out.output.contains("All::SubDeck2"));
    assert!(out.output.contains("SomeDeck"));
    assert!(out.output.contains("Default"));
}

#[test]
fn new_database_schema_list_models() {
    let out = run_ankitool_on_setup("newDatabaseSchemaListModels", &["list-models"]).unwrap();
    assert!(out.output.contains("SomeModel"));
    assert!(out.output.contains("SomeModel2"));
    assert!(out.output.contains("SomeModel3"));
    assert!(out.output.contains("SomeModel4"));
    assert!(out.output.contains("SomeModel4"));
}

#[test]
fn new_database_schema_list_fields() {
    let out = run_ankitool_on_setup("newDatabaseSchemaListFields", &["list-fields", "SomeModel"]).unwrap();
    assert!(out.output.contains("Front"));
    assert!(out.output.contains("Back"));
    assert!(out.output.contains("Add Reverse"));
}

#[test]
fn new_database_schema_list_templates() {
    let out = run_ankitool_on_setup("newDatabaseSchemaListFields", &["list-templates", "SomeModel"]).unwrap();
    assert!(out.output.contains("Card 1"));
    assert!(out.output.contains("Card 2"));
}

fn run_ankitool_on_setup(setup_name: &str, args: &[&str]) -> Result<TestOutput> {
    let env = setup_test(
        get_ankitool_executable(),
        &Path::new(TEST_SETUPS_PATH),
        setup_name,
    );
    let db_path = env.dir.path().join(&DEFAULT_ANKI_SOURCE_COLLECTION_NAME);
    let abs_path = db_path.canonicalize()?;

    let mut new_args = vec![abs_path.to_str().unwrap()];
    new_args.extend_from_slice(args);
    let output = get_shell_command_output(env.executable.to_str().unwrap(), &new_args);
    let out = TestOutput {
        env: env,
        success: output.0,
        output: output.1,
        stderr: output.2,
    };
    println!("STDOUT:\n{}", out.output);
    println!("STDERR:\n{}", out.stderr);
    check_same_notes_and_cards(
        &out.env.dir.path().join(DEFAULT_ANKI_SOURCE_COLLECTION_NAME),
        &out.env.dir.path().join(DEFAULT_ANKI_TARGET_COLLECTION_NAME),
    )?;
    Ok(out)
}
