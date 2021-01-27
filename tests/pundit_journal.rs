pub mod setup;

use anyhow::Result;
use setup::TestArg;
use setup::{run_pundit_diff, TestArg::NormalArg};
use std::path::Path;

use pundit::dir_utils::get_folders;
use setup::run_pundit_on_setup;

#[test]
fn test_yesterday() -> Result<()> {
    Ok(())
    // let out = run_pundit_on_setup(
    //     "journalCreateDirIfNonExistent",
    //     &[
    //         NormalArg("journal"),
    //         NormalArg("work"),
    //         NormalArg("yesterday"),
    //     ],
    // );
    // assert!(out.success);
    // dbg!(out.env.dir.path());
    // for file in get_folders(out.env.dir.path())? {
    //     dbg!(&file);
    // }
    // assert!(false);
    // Ok(())
    // assert!(out.output.lines().any(|line| line == "linkNote2"));
    // assert!(out.output.lines().all(|line| line != ""));
}
