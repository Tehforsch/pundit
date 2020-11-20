pub mod setup;

use std::path::Path;

use setup::{run_setup_with_args, TestOutput};

pub static TEST_SETUPS_PATH: &str = "testSetupsPundit";

#[test]
fn test_read_notes() {
    let out = run_pundit_on_setup("3linkedNotes", &["list"]);
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "note1"));
    assert!(out.output.lines().any(|line| line == "linkNote1"));
    assert!(out.output.lines().any(|line| line == "linkNote2"));
}

pub fn run_pundit_on_setup(setup_name: &str, args: &[&str]) -> TestOutput {
    run_setup_with_args(Path::new(TEST_SETUPS_PATH), setup_name, &args)
}
