use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use pathdiff::diff_paths;

fn traverse_folder_files(folder: &Path) -> Result<Box<dyn Iterator<Item = PathBuf>>> {
    let folder_files = Box::new(iter_files(folder)?);
    let sub_folders = iter_folders(folder)?;
    let sub_folder_results = sub_folders.map(|f| traverse_folder_files(&f));
    let sub_folder_files_iterators_result: Result<Vec<Box<dyn Iterator<Item = PathBuf>>>> =
        sub_folder_results.collect();
    let sub_folder_files_iterator = (sub_folder_files_iterators_result?)
        .into_iter()
        .flat_map(|it| it);
    Ok(Box::new(folder_files.chain(sub_folder_files_iterator)))
}

fn iter_files(folder: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    get_entries_with_predicate(folder, Path::is_file)
}

fn iter_folders(folder: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    get_entries_with_predicate(folder, Path::is_dir)
}

fn get_entries_with_predicate<F>(
    folder: &Path,
    predicate: F,
) -> Result<impl Iterator<Item = PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    let entries = fs::read_dir(folder)?;
    let dir_entries: std::io::Result<Vec<DirEntry>> = entries.collect();
    Ok(dir_entries?
        .into_iter()
        .map(|entry| entry.path())
        .filter(move |path| predicate(path)))
}

pub fn get_files_recursively(folder: &Path) -> Result<Vec<PathBuf>> {
    Ok(traverse_folder_files(folder)?.collect())
}

pub fn get_files(folder: &Path) -> Result<Vec<PathBuf>> {
    Ok(iter_files(folder)?.collect())
}

pub fn get_folders(folder: &Path) -> Result<Vec<PathBuf>> {
    Ok(iter_folders(folder)?.collect())
}

pub fn get_relative_path(folder: &Path, base_folder: &Path) -> Result<PathBuf> {
    diff_paths(folder, base_folder).ok_or_else(|| {
        anyhow!(format!(
            "Failed to construct relative link from {:?} to {:?}",
            folder, base_folder
        ))
    })
}

pub fn create_folder(folder: &Path) -> Result<()> {
    fs::create_dir_all(folder).context("While creating folder")
}
