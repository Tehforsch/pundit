pub mod setup;

use std::path::Path;

use setup::{run_setup_with_args, TestEnv};

pub static TEST_SETUPS_PATH: &str = "testSetupsPundit";

#[test]
fn test_read_notes() {
    let (_env, output) = run_pundit_on_setup("3linkedNotes", &["list"]);
    assert!(output.lines().any(|line| line == "note1"));
    assert!(output.lines().any(|line| line == "linkNote1"));
    assert!(output.lines().any(|line| line == "linkNote2"));
}

pub fn run_pundit_on_setup(setup_name: &str, args: &[&str]) -> (TestEnv, String) {
    run_setup_with_args(Path::new(TEST_SETUPS_PATH), setup_name, &args)
}
