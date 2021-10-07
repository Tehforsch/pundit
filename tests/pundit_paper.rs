pub mod setup;

use setup::run_pundit_on_setup;
use setup::TestArg::NormalArg;
use setup::TestArg::RelativePath;

#[test]
fn read_papers() {
    let out = run_pundit_on_setup(
        "paper",
        &[
            NormalArg("paper"),
            RelativePath("library.bib"),
            NormalArg("list"),
        ],
    );
    assert!(out.success);
    assert_eq!(
        out.output,
        "mustermannFirstPaper2020\nmustermannSecondPaper2020\n"
    );
}
