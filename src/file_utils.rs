use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

pub fn append_to_file(filename: &Path, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(filename)?;

    writeln!(file, "{}", content).context(format!("While appending to file {:?}", filename))?;
    Ok(())
}
