pub mod setup;

use std::path::Path;

use setup::TestArg::{NormalArg, RelativePath};

use setup::{run_pundit_on_setup, TestEnv};

#[test]
fn read_notes() {
    let out = run_pundit_on_setup("3linkedNotes", &[NormalArg("list")]);
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "note1"));
    assert!(out.output.lines().any(|line| line == "linkNote2"));
    assert!(out.output.lines().all(|line| line != ""));
}

#[test]
fn backlinks() {
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
fn link() {
    let out = run_pundit_on_setup(
        "3linkedNotes",
        &[
            NormalArg("show-link"),
            RelativePath("20200424162439-linkNote1.org"),
            RelativePath("20200424162358-note1.org"),
        ],
    );
    let line = out.output.lines().next().unwrap();
    assert!(out.success);
    assert_eq!(line, "[[file:20200424162358-note1.org][note1]]");

    let out = run_pundit_on_setup(
        "multiDirSetup",
        &[
            NormalArg("show-link"),
            RelativePath("20200424162358-note1.org"),
            RelativePath("subdir/20200424162453-linkNote2.org"),
        ],
    );
    let line = out.output.lines().next().unwrap();
    assert!(out.success);
    assert_eq!(
        line,
        "[[file:subdir/20200424162453-linkNote2.org][linkNote2]]"
    );

    let out = run_pundit_on_setup(
        "multiDirSetup",
        &[
            NormalArg("show-link"),
            RelativePath("subdir/20200424162453-linkNote2.org"),
            RelativePath("20200424162358-note1.org"),
        ],
    );
    let line = out.output.lines().next().unwrap();
    assert!(out.success);
    assert_eq!(line, "[[file:../20200424162358-note1.org][note1]]");
}

#[test]
fn new() {
    let out = run_pundit_on_setup("newNote", &[NormalArg("new"), NormalArg("newTitle")]);
    let filename = Path::new(out.output.lines().next().unwrap());
    assert!(filename.exists());
    assert_eq!(filename.parent().unwrap(), out.env.dir.path());
    assert!(out.success);
    // assert_eq!(line, "[[file:../20200424162358-note1.org][note1]]");
}

#[test]
fn graph() {
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
fn multi_dir_setup() {
    let out = run_pundit_on_setup(
        "multiDirSetup",
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
fn filter_subdir() {
    let out = run_pundit_on_setup(
        "multiDirSetup",
        &[
            NormalArg("list"),
            NormalArg("filter-subfolders"),
            NormalArg("subdir"),
        ],
    );
    assert!(out.success);
    assert!(out.output.lines().any(|line| line == "note1"));
    assert!(out.output.lines().any(|line| line == "linkNote1"));
    assert!(out.output.lines().all(|line| line != "linkNote2"));
}

pub fn get_abs_path_of_note(env: TestEnv, note_filename: &str) -> String {
    env.dir
        .path()
        .join(note_filename)
        .to_str()
        .expect("Converting note name to path")
        .to_owned()
}
