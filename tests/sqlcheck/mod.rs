use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;

use crate::setup::get_shell_command_output;

/// Check that the two databases only differ in modification timestamps
pub fn check_same_notes_and_cards(database1: &Path, database2: &Path) -> Result<()> {
    let output = get_sql_diff(database1, database2, &["cards", "notes"]);
    println!("sqldiff output: {}", &output);
    for line in output.lines() {
        let mut words = line.split_whitespace();
        let instruction1 = words.next().ok_or(anyhow!("Not the same database"))?;
        let _database = words.next().ok_or(anyhow!("Not the same database"))?;
        let instruction2 = words.next().ok_or(anyhow!("Not the same database"))?;
        assert_eq!("UPDATE", instruction1);
        assert_eq!("SET", instruction2);
        loop {
            let next_word = words.next();
            if let Some(word) = next_word {
                // End of update statement
                if word == "WHERE" {
                    break;
                }
            }
            let mut key_value_split = next_word
                .ok_or(anyhow!("Not the same database"))?
                .split("=");
            let key = key_value_split
                .next()
                .ok_or(anyhow!("Not the same database"))?;
            let _value = key_value_split
                .next()
                .ok_or(anyhow!("Not the same database"))?;
            assert_eq!("mod", key); // Modification timestamp
        }
    }
    Ok(())
}

pub fn get_sql_diff(database1: &Path, database2: &Path, tables: &[&str]) -> String {
    let mut args = vec![];
    for table in tables {
        args.extend_from_slice(&["--table", table]);
    }
    args.extend_from_slice(&[database1.to_str().unwrap(), database2.to_str().unwrap()]);
    let (_, output, _stderr) = get_shell_command_output("sqldiff", &args);
    output
}
