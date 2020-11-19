pub mod setup;

use setup::run_pundit_on_setup;

#[test]
fn test_read_notes() {
    let (_env, output) = run_pundit_on_setup("3linkedNotes", &["list"]);
    assert!(output.lines().any(|line| line == "note1"));
    assert!(output.lines().any(|line| line == "linkNote1"));
    assert!(output.lines().any(|line| line == "linkNote2"));
}
