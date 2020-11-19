use std::path::Path;

pub mod setup;

use setup::{run_setup_with_args, TestEnv};

pub static TEST_SETUPS_PATH: &str = "testSetupsPankit";
pub static DEFAULT_ANKI_COLLECTION_NAME: &str = "collection.anki2";

#[test]
fn test_read_database() {
    let (_env, output) = run_pankit_on_setup("emptyCollection", &[]);
    println!("{}", &output);
    // assert!(output.lines().any(|line| line == "note1"));
    // assert!(output.lines().any(|line| line == "linkNote1"));
    // assert!(output.lines().any(|line| line == "linkNote2"));
}

pub fn run_pankit_on_setup(setup_name: &str, args: &[&str]) -> (TestEnv, String) {
    let mut new_args = vec!["pankit", DEFAULT_ANKI_COLLECTION_NAME];
    new_args.extend_from_slice(args);
    run_setup_with_args(Path::new(TEST_SETUPS_PATH), setup_name, &new_args)
}
