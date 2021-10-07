use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use log::info;
use regex::Regex;

use crate::config;
use crate::fzf::select_interactively;
use crate::named::Named;
use crate::note::Note;
use crate::note_utils::find_or_create_note_with_special_content;
use crate::notes::Notes;
use crate::paper_opts::PaperOpts;
use crate::paper_opts::PaperSubCommand;

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
    let file_contents =
        fs::read_to_string(file).context(format!("While reading bibtex file at {:?}", &file))?;
    Ok(get_citekeys_from_contents(&file_contents))
}

fn get_citekeys_from_contents(contents: &str) -> Vec<String> {
    let re =
        Regex::new(r"@(article|inproceedings|book|inbook|proceedings|phdthesis)\{(\w*),").unwrap();
    re.captures_iter(contents)
        .map(|capture| capture.get(2).unwrap().as_str().to_owned())
        .collect()
}

fn find_note_for_cite_key<'a>(notes: &'a mut Notes, citekey: &str) -> Result<&'a Note> {
    create_new_paper_note_from_title(notes, citekey)
}

fn get_paper_base_note(notes: &Notes) -> Result<&Note> {
    notes.find_by_title(config::PAPER_NOTE_TITLE).ok_or_else(|| anyhow!("No note with title '{}' found in notes. Create one so that the new paper note can link to it.", config::PAPER_NOTE_TITLE))
}

fn get_paper_folder(notes: &Notes) -> Result<PathBuf> {
    let paper_folder = notes.folder.join(config::PAPER_FOLDER_NAME);
    if !paper_folder.is_dir() {
        return Err(anyhow!(
            "No folder '{}' found in notes folder.",
            config::PAPER_FOLDER_NAME
        ));
    }
    Ok(paper_folder)
}

fn create_new_paper_note_from_title<'a>(notes: &'a mut Notes, citekey: &str) -> Result<&'a Note> {
    let paper_note = get_paper_base_note(notes)?;
    let paper_folder = get_paper_folder(notes)?;
    let cite_string = format!("cite:{}", citekey);
    let link_text = paper_note.get_link_from_folder(&paper_folder)?;
    let additional_content = format!("\n{}\n{}", &link_text, cite_string);
    let title = citekey;
    let target_note = find_or_create_note_with_special_content(
        notes,
        &paper_folder,
        &title,
        &additional_content,
    )?;
    Ok(target_note)
}

impl Named for String {
    fn get_name(&self) -> &str {
        self
    }
}
