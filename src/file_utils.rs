use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn append_to_file(filename: &Path, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(filename)?;

    writeln!(file, "{}", content).context(format!("While appending to file {:?}", filename))?;
    Ok(())
}
