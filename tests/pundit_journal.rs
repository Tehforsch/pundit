pub mod setup;

use regex::Regex;
use setup::{TestArg::NormalArg, TestArg::RelativePath};
use std::fs;

use pundit::dir_utils::{get_files, get_folders};
use setup::run_pundit_on_setup;

#[test]
fn yesterday() {
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

#[test]
fn previous() {
    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("previous"),
            RelativePath("work/20210000000000-work_2021_02_01.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-01-31\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("previous"),
            RelativePath("work/20210000000000-work_2021_02_03.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-01\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("previous"),
            RelativePath("work/20210000000000-work_2021_01_30.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-01-30\n");
}

#[test]
fn next() {
    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("next"),
            RelativePath("work/20210000000000-work_2021_02_01.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-03\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("next"),
            RelativePath("work/20210000000000-work_2021_01_31.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-01\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("next"),
            RelativePath("work/20210000000000-work_2021_02_03.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-03\n");
}

#[test]
fn day_before() {
    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("day-before"),
            RelativePath("work/20210000000000-work_2021_02_01.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-01-31\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("day-before"),
            RelativePath("work/20210000000000-work_2021_02_03.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-02\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("day-before"),
            RelativePath("work/20210000000000-work_2021_01_30.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-01-29\n");
}

#[test]
fn day_after() {
    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("day-after"),
            RelativePath("work/20210000000000-work_2021_01_31.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-01\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("day-after"),
            RelativePath("work/20210000000000-work_2021_02_01.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-02\n");

    let out = run_pundit_on_setup(
        "journal",
        &[
            NormalArg("journal"),
            NormalArg("--date"),
            NormalArg("work"),
            NormalArg("day-after"),
            RelativePath("work/20210000000000-work_2021_02_03.org"),
        ],
    );
    assert!(out.success);
    assert_eq!(&out.output, "2021-02-04\n");
}
