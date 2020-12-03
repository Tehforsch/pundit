pub mod setup;

use setup::TestArg;
use setup::TestArg::{NormalArg, RelativePath};
use std::path::Path;

use setup::{
    get_pundit_executable, run_pundit_on_setup_with_args, show_output, TestEnv, TestOutput,
};

pub static TEST_SETUPS_PATH: &str = "testSetupsPundit";

#[test]
fn test_read_notes() {
    let out = run_pundit_on_setup("3linkedNotes", &[NormalArg("list")]);
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "note1"));
    assert!(out.output.lines().any(|line| line == "linkNote2"));
    assert!(out.output.lines().all(|line| line != ""));
}

#[test]
fn test_backlinks() {
    let out = run_pundit_on_setup(
        "database",
        &[
            NormalArg("list-backlinks"),
            RelativePath("20200424162358-note1.org"),
        ],
    );
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "linkNote1"));
    assert!(out.output.lines().any(|line| line == "linkNote2"));
}

#[test]
fn test_database() {

    // let env = setup_test(
    //     get_pundit_executable(),
    //     Path::new(TEST_SETUPS_PATH),
    //     "3linkedNotes",
    // );
    // let db_path = env.dir.path().join("pundit.yaml");
    // let abs_path = path.to_str().unwrap();
    // let out = run_pundit_on_setup_with_note_name(
    //     "3linkedNotes",
    //     &["pundit.yaml", "list-backlinks"],
    //     "20200424162358-note1.org",
    // );
    // let mut new_args = [abs_path, "list-backlinks", "20200424162358-note1.org"];
    // let out = run_pundit_on_env_with_args(env, &new_args);
    // show_output(&out);
    // out
    // assert!(out.success);
    // assert!(out.output.lines().any(|line| line == "linkNote1"));
    // assert!(out.output.lines().any(|line| line == "linkNote2"));
}

#[test]
fn test_graph() {
    let out = run_pundit_on_setup(
        "graph",
        &[NormalArg("list-graph"), RelativePath("linkNote1.org")],
    );
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "linkNote1"));
    assert!(out.output.lines().any(|line| line == "linkNote2"));
    for name in [
        "linkNote3.org",
        "linkNote4.org",
        "linkNote5.org",
        "linkNote6.org",
    ]
    .iter()
    {
        let out = run_pundit_on_setup("graph", &[NormalArg("list-graph"), RelativePath(name)]);
        assert!(out.success);
        assert!(out.output.lines().any(|line| line == "linkNote3"));
        assert!(out.output.lines().any(|line| line == "linkNote4"));
        assert!(out.output.lines().any(|line| line == "linkNote5"));
        assert!(out.output.lines().any(|line| line == "linkNote6"));
    }
}

#[test]
fn test_multi_dir_setup() {
    let out = run_pundit_on_setup(
        "multiDirSetup",
        &[
            NormalArg("--multidir"),
            NormalArg("list-backlinks"),
            RelativePath("20200424162358-note1.org"),
        ],
    );
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "linkNote1"));
    assert!(out.output.lines().any(|line| line == "linkNote2"));
}

pub fn get_abs_path_of_note(env: TestEnv, note_filename: &str) -> String {
    env.dir
        .path()
        .join(note_filename)
        .to_str()
        .expect("Converting note name to path")
        .to_owned()
}

pub fn run_pundit_on_setup(setup_name: &str, args: &[TestArg]) -> TestOutput {
    let out = run_pundit_on_setup_with_args(
        get_pundit_executable(),
        Path::new(TEST_SETUPS_PATH),
        setup_name,
        &args,
    )
    .unwrap();
    show_output(&out);
    out
}
