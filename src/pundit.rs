use anyhow::{anyhow, Context, Result};
use notes::read_notes;
use notes::Notes;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

pub mod anki;
pub mod args;
pub mod config;
pub mod note;
pub mod notes;
pub mod pankit;

use crate::args::{Opts, SubCommand};
use crate::config::NOTE_EXTENSION;
use crate::note::Note;
use clap::Clap;

use std::io::Write;
use std::process::{Command, Stdio};

use std::env::{current_dir, set_current_dir};

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    let entry_folder = current_dir()?;
    let note_folder = args.folder.canonicalize()?;
    set_current_dir(&note_folder)?;
    let notes = read_notes(&PathBuf::from("."))?;
    run(&entry_folder, &note_folder, args, &notes)?;
    Ok(())
}

fn get_notes<'a>(notes: &'a Notes, filter_str: Option<&'a str>) -> impl Iterator<Item = &'a Note> {
    match filter_str {
        None => get_notes_filtered(notes, ""),
        Some(s) => get_notes_filtered(notes, s),
    }
}

fn get_notes_filtered<'a>(notes: &'a Notes, filter_str: &'a str) -> impl Iterator<Item = &'a Note> {
    let cloned_str = filter_str.to_owned();
    notes
        .iter()
        .filter(move |note| note.title.contains(&cloned_str))
}

fn list_notes(notes: &Notes, filter_str: Option<&str>) {
    for note in get_notes(notes, filter_str) {
        println!("{}", note.title);
    }
}

fn list_backlinks(notes: &Notes, note: &Note) {
    for link in get_backlinks(notes, note) {
        println!("{}", link.title);
    }
}

fn get_backlinks<'a>(notes: &'a Notes, note: &'a Note) -> impl Iterator<Item = &'a Note> {
    let selected_filename = note.filename.file_name().unwrap();
    notes.iter().filter(move |n| {
        n.links
            .iter()
            // TODO: This does not work for multi-dir setups!
            .any(|link| notes.get(*link).filename.file_name().unwrap() == selected_filename)
    })
}

fn find_backlinked_note_interactively(notes: &Notes, note: &Note) {
    let backlinks = get_backlinks(notes, note);
    let backlinks_coll: Vec<&Note> = backlinks.collect();
    // backlinks_coll = ();
    select_note_interactively(&backlinks_coll);
}

fn find_note_interactively(notes: &Notes, filter_str: Option<&str>) {
    let notes_filtered = get_notes(notes, filter_str);
    let notes_filtered_coll: Vec<&Note> = notes_filtered.collect();
    select_note_interactively(&notes_filtered_coll);
}

fn select_note_interactively(notes: &[&Note]) {
    let note = select_note_with_fzf(notes);
    // For interactive use from other processes: Print the filename of the resulting file.
    match note {
        Some(n) => println!("{}", n.filename.canonicalize().unwrap().to_str().unwrap()),
        None => println!(),
    };
}

fn get_link_interactively(notes: &Notes, filter_str: Option<&str>) {
    let notes_filtered = get_notes(notes, filter_str);
    let notes_filtered_coll: Vec<&Note> = notes_filtered.collect();
    let note = select_note_with_fzf(&notes_filtered_coll);
    if let Some(n) = note {
        println!("{}", n.get_link());
    }
}

fn select_note_with_fzf(notes: &[&Note]) -> Option<Note> {
    let strs: Vec<String> = notes
        .iter()
        .enumerate()
        .map(|(i, note)| format!("{};{};{}", i, note.title, note.filename.to_str().unwrap()))
        .collect();
    let content = strs.join("\n");
    let output = run_fzf_on_string(&content);
    let split: Vec<&str> = output.split('\n').collect();
    let query = split[0];
    let key = split[1];
    let note_info = split[2];
    let note_info_split: Vec<&str> = note_info.split(';').collect();
    if key != "" || note_info_split.len() != 3 {
        let new_note_title = query.replace("\n", "");
        let note = Note::from_title_and_date(&new_note_title);
        note.write_without_contents().expect("Failed to write note");
        Some(note)
    } else {
        let index = note_info_split[0].parse::<usize>().unwrap();
        let note = notes[index];
        assert_eq!(note.filename.to_str().unwrap(), note_info_split[2]);
        Some((*note).clone())
    }
}

fn run_fzf_on_string(content: &str) -> String {
    let mut child = Command::new("fzf")
        .args(&[
            "--print-query",
            "--margin=1,0",
            "--with-nth=2",
            "--delimiter=;",
            // "--preview=cat '{3}'",
            "--preview=",
            "--expect=ctrl-t",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn fzf");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    str::from_utf8(&output.stdout)
        .expect("Failed to decode fzf output as utf8")
        .to_owned()
}

fn delete_file(filename: &Path) {
    println!("Deleting {}", filename.to_str().unwrap());
}

fn delete_note(notes: &Notes, note: &Note) {
    let mut backlink_notes = get_backlinks(notes, note);
    let next = backlink_notes.next();
    match next {
        None => delete_file(&note.filename),
        Some(note) => {
            println!("There are links to this note: ");
            println!("\t{}", note.title);
            for backlink_note in backlink_notes {
                println!("\t{}", backlink_note.title);
            }
            println!("Not deleting note.");
        }
    }
}

fn get_args() -> Opts {
    Opts::parse()
}

fn transform_passed_path(entry_folder: &Path, note_folder: &Path, path: &Path) -> Result<PathBuf> {
    let absolute_path = entry_folder.join(path).canonicalize().context(anyhow!(
        "Finding file passed as argument: {}",
        path.to_str().unwrap()
    ))?;
    Ok(absolute_path
        .strip_prefix(note_folder.canonicalize().unwrap())
        .map_err(|_| anyhow!("Note not in folder: {}", path.to_str().unwrap()))?
        .to_path_buf())
}

fn find_by_filename<'a>(
    notes: &'a Notes,
    entry_folder: &Path,
    note_folder: &Path,
    filename: &Path,
) -> Result<&'a Note> {
    let transformed = transform_passed_path(entry_folder, note_folder, filename)?;
    println!("{:?}", transformed);
    notes
        .find_by_filename(&transformed)
        .ok_or_else(|| anyhow!("Given note not found: {}", filename.to_str().unwrap()))
}

fn run(entry_folder: &Path, note_folder: &Path, args: Opts, notes: &Notes) -> Result<()> {
    match args.subcmd {
        SubCommand::List(l) => {
            list_notes(notes, l.filter.as_deref());
        }
        SubCommand::ListBacklinks(l) => {
            let note = find_by_filename(notes, entry_folder, note_folder, &l.filename)?;
            list_backlinks(&notes, &note);
        }
        SubCommand::Backlinks(l) => {
            let note = find_by_filename(notes, entry_folder, note_folder, &l.filename)?;
            find_backlinked_note_interactively(&notes, note);
        }
        SubCommand::Link(l) => {
            get_link_interactively(&notes, l.filter.as_deref());
        }
        SubCommand::Find(l) => {
            find_note_interactively(&notes, l.filter.as_deref());
        }
        SubCommand::Rename(l) => {
            todo!();
            // let note = find_by_filename(notes, entry_folder, note_folder, &l.filename)?;
            // rename_note(&notes, &note, &l.new_name);
        }
        SubCommand::Delete(l) => {
            let note = find_by_filename(notes, entry_folder, note_folder, &l.filename)?;
            delete_note(&notes, &note);
        }
        SubCommand::Pankit(l) => {
            crate::pankit::update_anki(&l.database, &l.pankit_db, &notes, l.conflict_handling)?
        }
    }
    Ok(())
}
