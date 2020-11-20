use std::path::Path;

pub mod setup;

use setup::{get_shell_command_output, run_setup_with_args, TestEnv};

pub static TEST_SETUPS_PATH: &str = "testSetupsPankit";
pub static DEFAULT_ANKI_SOURCE_COLLECTION_NAME: &str = "source.anki2";
pub static DEFAULT_ANKI_TARGET_COLLECTION_NAME: &str = "target.anki2";

#[test]
fn test_read_database() {
    let (_env, output) = run_pankit_on_setup("emptyCollection", &[]);
    println!("{}", &output);
    // assert!(output.lines().any(|line| line == "note1"));
    // assert!(output.lines().any(|line| line == "linkNote1"));
    // assert!(output.lines().any(|line| line == "linkNote2"));
}

/// Check that the two databases only differ in modification timestamps
fn assert_same_database(database1: &Path, database2: &Path) -> String {
    let output = get_shell_command_output(
        "sqldiff",
        &[database1.to_str().unwrap(), database2.to_str().unwrap()],
    );
    for line in output.lines() {
        let mut words = line.split_whitespace();
        let instruction1 = words.next().unwrap();
        let _database = words.next().unwrap();
        let instruction2 = words.next().unwrap();
        let mut keyValueSplit = words.next().unwrap().split("=");
        let key = keyValueSplit.next().unwrap();
        let _value = keyValueSplit.next().unwrap();
        assert_eq!("UPDATE", instruction1);
        assert_eq!("SET", instruction2);
        assert_eq!("mod", key); // Modification timestamp
    }
    "".to_string()
}

pub fn run_pankit_on_setup(setup_name: &str, args: &[&str]) -> (TestEnv, String) {
    let mut new_args = vec!["pankit", DEFAULT_ANKI_SOURCE_COLLECTION_NAME];
    new_args.extend_from_slice(args);
    let (env, output) = run_setup_with_args(Path::new(TEST_SETUPS_PATH), setup_name, &new_args);
    assert_same_database(
        &env.dir.path().join(DEFAULT_ANKI_SOURCE_COLLECTION_NAME),
        &env.dir.path().join(DEFAULT_ANKI_TARGET_COLLECTION_NAME),
    );
    (env, output)
}
