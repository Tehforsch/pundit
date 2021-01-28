use anyhow::{anyhow, Result};
use pundit::fzf::run_fzf;
use pundit::notes::read_notes;
use pundit::notes::Notes;
use std::error::Error;
use std::path::Path;

use clap::Clap;
use pundit::args::{Opts, SubCommand};
use pundit::graph::get_connected_component_undirected;
use pundit::note::{create_new_note_from_title, Note};

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    let note_folder = args.folder.canonicalize()?;
    let notes = read_notes(&note_folder, &args.database, !args.singledir)?;
    run(args, &notes)?;
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
    note.backlinks.iter().map(move |link| &notes[*link])
}

fn find_backlinked_note_interactively(notes: &Notes, folder: &Path, note: &Note) -> Result<()> {
    let backlinks = get_backlinks(notes, note);
    let backlinks_coll: Vec<&Note> = backlinks.collect();
    select_note_interactively(notes, folder, &backlinks_coll)
}

fn find_note_interactively(notes: &Notes, folder: &Path, filter_str: Option<&str>) -> Result<()> {
    let notes_filtered = get_notes(notes, filter_str);
    let notes_filtered_coll: Vec<&Note> = notes_filtered.collect();
    select_note_interactively(notes, folder, &notes_filtered_coll)
}

fn select_note_interactively(all_notes: &Notes, folder: &Path, notes: &[&Note]) -> Result<()> {
    let note = select_note_with_fzf(all_notes, folder, notes)?;
    // For interactive use from other processes: Print the filename of the resulting file.
    match note {
        Some(n) => n.show_filename(),
        None => println!(),
    };
    Ok(())
}

fn show_link(note1: &Note, note2: &Note) -> Result<()> {
    let link_text = note2.get_link_from(note1)?;
    println!("{}", link_text);
    Ok(())
}

fn show_link_interactively(
    notes: &Notes,
    folder: &Path,
    note_src: &Note,
    filter_str: Option<&str>,
) -> Result<()> {
    let notes_filtered = get_notes(notes, filter_str);
    let notes_filtered_coll: Vec<&Note> = notes_filtered.collect();
    let note = select_note_with_fzf(notes, folder, &notes_filtered_coll)?;
    if let Some(n) = note {
        show_link(note_src, &n)?;
    }
    Ok(())
}

fn select_note_with_fzf(all_notes: &Notes, folder: &Path, notes: &[&Note]) -> Result<Option<Note>> {
    let mut sorted_notes: Vec<&Note> = notes.to_vec();
    sorted_notes.sort_by(|n1, n2| n1.title.partial_cmp(&n2.title).unwrap());

    let strs: Vec<String> = sorted_notes
        .iter()
        .enumerate()
        .map(|(i, note)| format!("{};{};{}", i, note.title, note.filename.to_str().unwrap()))
        .collect();

    let content = strs.join("\n");
    let output = run_fzf_on_notes_string(&content);
    let split: Vec<&str> = output.split('\n').collect();
    let query = split[0];
    if split.len() == 1 {
        if query.trim_start_matches(" ") == "" {
            return Ok(None);
        }
        return Ok(Some(create_new_note_from_query(all_notes, folder, query)?));
    }
    let key = split[1];
    let note_info = split[2];
    let note_info_split: Vec<&str> = note_info.split(';').collect();
    if key != "" || note_info_split.len() != 3 {
        return Ok(Some(create_new_note_from_query(all_notes, folder, query)?));
    } else {
        let index = note_info_split[0].parse::<usize>().unwrap();
        let note = sorted_notes[index];
        assert_eq!(note.filename.to_str().unwrap(), note_info_split[2]);
        Ok(Some((*note).clone()))
    }
}

fn create_new_note_from_query(notes: &Notes, folder: &Path, query: &str) -> Result<Note> {
    let new_note_title = query.replace("\n", "");
    create_new_note_from_title(notes, folder, &new_note_title)
}

fn run_fzf_on_notes_string(content: &str) -> String {
    let args = &[
        "--print-query",
        "--margin=1,0",
        "--with-nth=2",
        "--delimiter=;",
        "--preview=",
        "--expect=ctrl-t",
    ];
    run_fzf(content, args)
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

fn run_find_graph(notes: &Notes, folder: &Path, note: &Note) -> Result<()> {
    let connected = get_connected_component_undirected(notes, note);
    select_note_interactively(notes, folder, &connected)
}

fn run_list_graph(notes: &Notes, note: &Note) {
    let connected = get_connected_component_undirected(notes, note);
    for n in connected.iter() {
        println!("{}", n.title);
    }
}

fn get_args() -> Opts {
    Opts::parse()
}

// fn transform_passed_path(entry_folder: &Path, note_folder: &Path, path: &Path) -> Result<PathBuf> {
//     let absolute_path = entry_folder.join(path).canonicalize().context(anyhow!(
//         "Finding file passed as argument: {}",
//         path.to_str().unwrap()
//     ))?;
//     Ok(absolute_path
//         .strip_prefix(note_folder.canonicalize().unwrap())
//         .map_err(|_| anyhow!("Note not in folder: {}", path.to_str().unwrap()))?
//         .to_path_buf())
// }

fn find_by_filename<'a>(notes: &'a Notes, filename: &Path) -> Result<&'a Note> {
    // let transformed = transform_passed_path(entry_folder, note_folder, filename)?;
    let transformed = filename.canonicalize()?;
    notes
        .find_by_filename(&transformed)
        .ok_or_else(|| anyhow!("Given note not found: {}", filename.to_str().unwrap()))
}

fn run(args: Opts, notes: &Notes) -> Result<()> {
    match args.subcmd {
        SubCommand::List(l) => {
            list_notes(notes, l.filter.as_deref());
        }
        SubCommand::ListBacklinks(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            list_backlinks(&notes, &note);
        }
        SubCommand::Backlinks(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            find_backlinked_note_interactively(&notes, &args.folder, note)?;
        }
        SubCommand::Link(l) => {
            let note1 = find_by_filename(notes, &l.note1)?;
            show_link_interactively(&notes, &args.folder, &note1, l.filter.as_deref())?;
        }
        SubCommand::ShowLink(l) => {
            let note1 = find_by_filename(notes, &l.note1)?;
            let note2 = find_by_filename(notes, &l.note2)?;
            show_link(&note1, &note2)?;
        }
        SubCommand::New(l) => {
            let note = create_new_note_from_title(notes, &args.folder, &l.title)?;
            note.show_filename();
        }
        SubCommand::Find(l) => {
            find_note_interactively(&notes, &args.folder, l.filter.as_deref())?;
        }
        SubCommand::Rename(_) => {
            todo!();
            // let note = find_by_filename(notes, &l.filename)?;
            // rename_note(&notes, &note, &l.new_name);
        }
        SubCommand::Delete(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            delete_note(&notes, &note);
        }
        SubCommand::Graph(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            run_find_graph(notes, &args.folder, note)?;
        }
        SubCommand::ListGraph(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            run_list_graph(notes, note);
        }
        SubCommand::Pankit(l) => {
            pundit::pankit::update_anki(&l.database, &l.pankit_db, &notes, l.conflict_handling)?
        }
        SubCommand::PankitGetNote(l) => pundit::pankit::pankit_get_note(&l.database)?,
        SubCommand::Journal(l) => {
            pundit::journal::run_journal(&notes, &args.folder, &l)?;
        }
    }
    Ok(())
}
