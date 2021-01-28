use std::path::Path;

use clap::Clap;
use pathdiff::diff_paths;

use crate::{dir_utils::get_relative_path, note::Note};
/// Various options for filtering lists of notes
#[derive(Clap, Debug)]
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
        .any(|f| {
            dbg!(&f);
            note_is_in_subfolder(base_folder, &f, note)
        })
}

fn note_is_in_subfolder(base_folder: &Path, subfolder: &Path, note: &Note) -> bool {
    let parent_folder = note.filename.parent().unwrap();
    let relative_path = get_relative_path(parent_folder, base_folder).unwrap();
    dbg!(&parent_folder, &relative_path, &base_folder, &subfolder);
    relative_path == subfolder
}

#[derive(Clap, Debug)]
pub struct FilterSubfolders {
    filter: Vec<String>,
}
