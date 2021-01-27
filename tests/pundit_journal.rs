pub mod setup;

use anyhow::Result;
use regex::Regex;
use setup::TestArg;
use setup::{run_pundit_diff, TestArg::NormalArg};
use std::{fs, path::Path};

use pundit::dir_utils::{get_files, get_folders};
use setup::run_pundit_on_setup;

#[test]
fn test_yesterday() {
    let out = run_pundit_on_setup(
        "journalCreateDirIfNonExistent",
        &[
            NormalArg("journal"),
            NormalArg("work"),
            NormalArg("yesterday"),
        ],
    );
    assert!(out.success);
    let folders = get_folders(out.env.dir.path()).unwrap();
    assert_eq!(folders.len(), 1);
    let folder = folders.iter().next().unwrap();
    assert_eq!(folder.file_name().unwrap(), "work");
    let files = get_files(folder).unwrap();
    assert_eq!(files.len(), 2);
    let entry_file = files
        .iter()
        .max_by_key(|f| f.file_name().unwrap().to_str().unwrap().len())
        .unwrap(); // Longest file is the new entry, shorter one is the main journal file
    let contents = fs::read_to_string(entry_file).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    println!("{}", lines[1]);
    let re = Regex::new(r"\[\[file:\d{14}-work.org\]\[work\]\]").unwrap();
    assert!(re.is_match(lines[1]));
    // assert_eq!(file, "work");
}
