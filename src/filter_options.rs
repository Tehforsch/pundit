use std::path::Path;

use clap::Parser;

use crate::dir_utils::get_relative_path;
use crate::note::Note;
/// Various options for filtering lists of notes
#[derive(Parser, Debug)]
pub enum FilterOptions {
    IncludeAll,
    FilterSubfolders(FilterSubfolders),
}

impl FilterOptions {
    pub fn includes_note(&self, base_folder: &Path, note: &Note) -> bool {
        match self {
            FilterOptions::IncludeAll => true,
            FilterOptions::FilterSubfolders(subfolders) => {
                !note_is_in_any_subfolder(base_folder, note, subfolders)
            }
        }
    }
}

fn note_is_in_any_subfolder(
    base_folder: &Path,
    note: &Note,
    subfolders: &FilterSubfolders,
) -> bool {
    subfolders
        .filter
        .iter()
        .map(|f_name| Path::new(f_name))
        .any(|f| note_is_in_subfolder(base_folder, &f, note))
}

fn note_is_in_subfolder(base_folder: &Path, subfolder: &Path, note: &Note) -> bool {
    let parent_folder = note.filename.parent().unwrap();
    let relative_path = get_relative_path(parent_folder, base_folder).unwrap();
    relative_path == subfolder
}

#[derive(Parser, Debug)]
pub struct FilterSubfolders {
    filter: Vec<String>,
}
