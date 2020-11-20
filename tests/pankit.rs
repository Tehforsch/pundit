use std::path::Path;

use anyhow::Result;

pub mod setup;

use setup::{get_shell_command_output, run_setup_with_args, TestEnv};

pub static TEST_SETUPS_PATH: &str = "testSetupsPankit";
pub static DEFAULT_ANKI_SOURCE_COLLECTION_NAME: &str = "source.anki2";
pub static DEFAULT_ANKI_TARGET_COLLECTION_NAME: &str = "target.anki2";

#[test]
fn test_all_setups() -> Result<()> {
    for dir in Path::new(TEST_SETUPS_PATH).read_dir()? {
        let test_name = dir?.path();
        if test_name.is_dir() {
            let test_name_str = test_name.file_name().unwrap().to_str().unwrap();
            println!("Running test {}", test_name_str);
            let (_env, _output) = run_pankit_on_setup(test_name_str, &[]);
        }
    }
    Ok(())
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
        let mut key_value_split = words.next().unwrap().split("=");
        let key = key_value_split.next().unwrap();
        let _value = key_value_split.next().unwrap();
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
