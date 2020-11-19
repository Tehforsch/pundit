pub mod setup;

use setup::{run_pundit, setup_test};

#[test]
fn test_read_database() {
    assert!(output.lines().any(|line| line == "note1"));
    assert!(output.lines().any(|line| line == "linkNote1"));
    assert!(output.lines().any(|line| line == "linkNote2"));
}
