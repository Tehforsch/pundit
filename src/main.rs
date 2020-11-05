use std::fs;
use std::str;
use std::io;
use std::path::{Path,PathBuf};

pub mod args;
pub mod config;
pub mod note;

use crate::args::{Opts, SubCommand};
use crate::note::Note;
use clap::Clap;

use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let args = get_args();
    let note_folder = match args.folder {
        None => PathBuf::from("test"),
        Some(ref f) => f.clone()
    };
    let notes = read_notes(&note_folder)?;
    run(args, notes);
    Ok(())
}

fn read_notes(note_folder: &Path) -> io::Result<Vec<Note>> {
    let mut notes = vec![];
    for entry in fs::read_dir(note_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            notes.push(Note::from_filename(&path));
        }
    }
    Ok(notes)
}

fn get_notes<'a>(notes: &'a[Note], filter_str: Option<&'a str>) -> impl Iterator<Item = &'a Note>{
    match filter_str {
        None => get_notes_filtered(notes, ""),
        Some(s) => get_notes_filtered(notes, s),
    }
}

fn get_notes_filtered<'a>(notes: &'a[Note], filter_str: &'a str) -> impl Iterator<Item = &'a Note>{
    let cloned_str = filter_str.to_owned();
    notes.iter().filter(move |note| note.title.contains(&cloned_str))
}


fn list_notes(notes: &[Note], filter_str: Option<&str>) {
    for note in get_notes(notes, filter_str) {
        println!("{}", note.title);
    }
}

fn list_backlinks(notes: &[Note], note: &Note) {
    for n in notes.iter() {
        for link in n.links.iter() {
            if link.title == note.title {
                println!("{}", link.title);
            }
        }
    }
}

fn find_note_interactively(notes: &[Note], filter_str: Option<&str>) {
    let notes = get_notes(notes, filter_str);
    let strs: Vec<String> = notes.map(|note| format!("{};{}", note.title, note.filename.to_str().unwrap())).collect();
    let content = strs.join("\n");
    let output = run_fzf_on_string(&content);
    dbg!(output);
}

fn run_fzf_on_string(content: &str) -> String {
    let mut child = Command::new("fzf")
        .args(&["--print-query", "--with-nth=1", "--delimiter=;", "--preview=bat '{2}'"])
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
    str::from_utf8(&output.stdout).expect("Failed to decode fzf output as utf8").to_owned()
}

pub fn get_args() -> Opts {
    Opts::parse()
}

pub fn run(args: Opts, notes: Vec<Note>) {
    match args.subcmd {
        SubCommand::List(l) => {
            list_notes(&notes, l.filter.as_deref());
        }
        SubCommand::Backlinks(l) => {
            let note = Note::from_filename(&l.filename);
            list_backlinks(&notes, &note);
        }
        SubCommand::Find(l) => {
            find_note_interactively(&notes, l.filter.as_deref());
        }
    }
}
