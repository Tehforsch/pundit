use std::{fs, path::Path};

use anyhow::{Context, Result};
use log::info;
use crate::{fzf::select_interactively, named::Named, note::{Note, create_new_note_from_title}, notes::Notes, paper_opts::{PaperOpts, PaperSubCommand}};
use regex::Regex;

pub fn run_paper(notes: &mut Notes, args: &PaperOpts) -> Result<()> {
    let bibtex_file = args.bibtex_file.canonicalize()?;
    match &args.subcmd {
        PaperSubCommand::Find => find_paper_note(notes, &bibtex_file),
        PaperSubCommand::List => list_papers(&bibtex_file),
    }
}


fn list_papers(bibtex_file: &Path) -> Result<()> {
    let citekeys = get_citekeys_from_file(bibtex_file)?;
    for citekey in citekeys.iter() {
        info!("{}", citekey);
    }
    Ok(())
}

fn find_paper_note(notes: &mut Notes, bibtex_file: &Path) -> Result<()> {
    let citekeys = get_citekeys_from_file(bibtex_file)?;
    let selected_citekey = select_interactively(&citekeys);
    if let Some(selected_citekey) = selected_citekey {
        let note = find_note_for_cite_key(notes, selected_citekey)?;
        note.show_filename();
    };
    Ok(())
}

fn get_citekeys_from_file(file: &Path) -> Result<Vec<String>> {
    let file_contents = fs::read_to_string(file).context(format!("While reading bibtex file at {:?}", &file))?;
    Ok(get_citekeys_from_contents(&file_contents))
}

fn get_citekeys_from_contents(contents: &str) -> Vec<String> {
    let re = Regex::new(r"@article\{(\w*),").unwrap();
    re.captures_iter(contents).map(|capture| capture.get(1).unwrap().as_str().to_owned()).collect()
}

fn find_note_for_cite_key(notes: &Notes, citekey: &str) -> Result<Note> {
    let note_with_citekey_as_title = notes.find_by_title(citekey);
    match note_with_citekey_as_title {
        Some(note) => Ok(note.clone()),
        None => create_new_note_from_title(notes, &notes.folder, citekey),
    }
}

impl Named for String {
    fn get_name(&self) -> &str {
        self
    }
}
