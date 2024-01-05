pub mod logger;

use std::error::Error;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser;
use log::error;
use log::info;
use logger::init_logger;
use pundit::args::Opts;
use pundit::args::SubCommand;
use pundit::filter_options::FilterOptions;
use pundit::fzf::run_fzf;
use pundit::graph::get_connected_component_undirected;
use pundit::note::create_new_note_from_title;
use pundit::note::Note;
use pundit::note_utils::get_backlinks;
use pundit::notes::read_notes;
use pundit::notes::Notes;
use pundit::settings::Settings;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = get_args();
    let settings = Settings::from_default_location();
    if let Some(mut settings) = settings {
        settings.expand_all_paths()?;
        update_args_with_settings(&mut args, &settings);
    }
    init_logger(args.add_identifier).unwrap();
    let note_folder = args.folder.as_ref().unwrap().canonicalize()?;
    let mut notes = read_notes(&note_folder, &args.database, !args.singledir)?;
    run(args, &mut notes)?;
    Ok(())
}

fn update_args_with_settings(args: &mut Opts, settings: &Settings) {
    args.folder = args.folder.clone().or(settings.pundit_folder.clone());
    if args.folder.is_none() {
        panic!("No pundit folder specified!");
    }
}

fn get_notes<'a>(
    notes: &'a Notes,
    filter: Option<FilterOptions>,
) -> impl Iterator<Item = &'a Note> {
    match filter {
        None => get_notes_filtered(notes, FilterOptions::IncludeAll),
        Some(s) => get_notes_filtered(notes, s),
    }
}

fn get_notes_filtered<'a>(
    notes: &'a Notes,
    filter: FilterOptions,
) -> impl Iterator<Item = &'a Note> {
    notes
        .iter()
        .filter(move |note| filter.includes_note(&notes.folder, note))
}

fn list_notes(notes: &Notes, filter: Option<FilterOptions>) {
    for note in get_notes(notes, filter) {
        info!("{}", note.title);
    }
}

fn list_backlinks(notes: &Notes, note: &Note, show_path: bool) {
    for link in get_backlinks(notes, note) {
        if show_path {
            link.show_filename();
        } else {
            info!("{}", link.title);
        }
    }
}

fn find_backlinked_note_interactively(notes: &Notes, note: &Note) -> Result<()> {
    let backlinks = get_backlinks(notes, note);
    let backlinks_coll: Vec<&Note> = backlinks.collect();
    select_note_interactively(notes, &backlinks_coll)
}

fn find_note_interactively(notes: &Notes, filter: Option<FilterOptions>) -> Result<()> {
    let notes_filtered = get_notes(notes, filter);
    let notes_filtered_coll: Vec<&Note> = notes_filtered.collect();
    select_note_interactively(notes, &notes_filtered_coll)
}

fn select_note_interactively(all_notes: &Notes, notes: &[&Note]) -> Result<()> {
    let note = select_note_with_fzf(all_notes, notes)?;
    // For interactive use from other processes: Print the filename of the resulting file.
    match note {
        Some(n) => n.show_filename(),
        None => info!(""),
    };
    Ok(())
}

fn show_link(note1: &Note, note2: &Note) -> Result<()> {
    let link_text = note2.get_link_from(note1)?;
    info!("{}", link_text);
    Ok(())
}

fn show_link_interactively(
    notes: &Notes,
    note_src: &Note,
    filter: Option<FilterOptions>,
) -> Result<()> {
    let notes_filtered = get_notes(notes, filter);
    let notes_filtered_coll: Vec<&Note> = notes_filtered.collect();
    let note = select_note_with_fzf(notes, &notes_filtered_coll)?;
    if let Some(n) = note {
        show_link(note_src, &n)?;
    }
    Ok(())
}

fn select_note_with_fzf(all_notes: &Notes, notes: &[&Note]) -> Result<Option<Note>> {
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
        return Ok(Some(create_new_note_from_query(all_notes, query)?));
    }
    let key = split[1];
    let note_info = split[2];
    let note_info_split: Vec<&str> = note_info.split(';').collect();
    if key != "" || note_info_split.len() != 3 {
        return Ok(Some(create_new_note_from_query(all_notes, query)?));
    } else {
        let index = note_info_split[0].parse::<usize>().unwrap();
        let note = sorted_notes[index];
        assert_eq!(note.filename.to_str().unwrap(), note_info_split[2]);
        Ok(Some((*note).clone()))
    }
}

fn create_new_note_from_query(notes: &Notes, query: &str) -> Result<Note> {
    let new_note_title = query.replace("\n", "");
    create_new_note_from_title(notes, &notes.folder, &new_note_title)
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
    info!("Deleting {}", filename.to_str().unwrap());
}

fn delete_note(notes: &Notes, note: &Note) {
    let mut backlink_notes = get_backlinks(notes, note);
    let next = backlink_notes.next();
    match next {
        None => delete_file(&note.filename),
        Some(note) => {
            error!("There are links to this note: ");
            error!("\t{}", note.title);
            for backlink_note in backlink_notes {
                error!("\t{}", backlink_note.title);
            }
            error!("Not deleting note.");
        }
    }
}

fn run_find_graph(notes: &Notes, note: &Note) -> Result<()> {
    let connected = get_connected_component_undirected(notes, note);
    select_note_interactively(notes, &connected)
}

fn run_list_graph(notes: &Notes, note: &Note) {
    let connected = get_connected_component_undirected(notes, note);
    for n in connected.iter() {
        info!("{}", n.title);
    }
}

fn get_args() -> Opts {
    Opts::parse()
}

fn find_by_filename<'a>(notes: &'a Notes, filename: &Path) -> Result<&'a Note> {
    let transformed = filename.canonicalize()?;
    notes
        .find_by_filename(&transformed)
        .ok_or_else(|| anyhow!("Given note not found: {}", filename.to_str().unwrap()))
}

fn run(args: Opts, mut notes: &mut Notes) -> Result<()> {
    match args.subcmd {
        SubCommand::List(l) => {
            list_notes(notes, l.filter);
        }
        SubCommand::ListBacklinks(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            list_backlinks(&notes, &note, l.show_path);
        }
        SubCommand::Backlinks(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            find_backlinked_note_interactively(&notes, note)?;
        }
        SubCommand::Link(l) => {
            let note1 = find_by_filename(notes, &l.note1)?;
            show_link_interactively(&notes, &note1, l.filter)?;
        }
        SubCommand::ShowLink(l) => {
            let note1 = find_by_filename(notes, &l.note1)?;
            let note2 = find_by_filename(notes, &l.note2)?;
            show_link(&note1, &note2)?;
        }
        SubCommand::New(l) => {
            let note = create_new_note_from_title(notes, &notes.folder, &l.title)?;
            note.show_filename();
        }
        SubCommand::Find(l) => {
            find_note_interactively(&notes, l.filter)?;
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
            run_find_graph(notes, note)?;
        }
        SubCommand::ListGraph(l) => {
            let note = find_by_filename(notes, &l.filename)?;
            run_list_graph(notes, note);
        }
        SubCommand::Pankit(l) => {
            pundit::pankit::update_anki(&l.database, &l.pankit_db, &notes, l.conflict_handling)?
        }
        SubCommand::PankitGetNote(l) => {
            pundit::pankit::pankit_get_note(&l.database, l.model_filename)?
        }
        SubCommand::Journal(l) => {
            pundit::journal::run_journal(&mut notes, &l)?;
        }
        SubCommand::Paper(l) => {
            pundit::paper::run_paper(&mut notes, &l)?;
        }
    }
    Ok(())
}
