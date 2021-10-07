// TAKEN FROM
// https://github.com/assert-rs/dir-diff/blob/master/src/lib.rs
// and modified because i need some output about where the difference is.

use std::cmp::Ordering;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use dissimilar::diff;
use itertools::Itertools;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// The various errors that can happen when diffing two directories
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    StripPrefix(std::path::StripPrefixError),
    WalkDir(walkdir::Error),
}

#[derive(Debug)]
pub enum Difference {
    Depth(DirEntry, DirEntry),
    FileType(DirEntry, DirEntry),
    FileName(DirEntry, DirEntry),
    FileContents(DirEntry, DirEntry),
    LeftoverEntriesA(DirEntry),
    LeftoverEntriesB(DirEntry),
}

impl Display for Difference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Difference::Depth(a, b) => {
                format!("Different depths in dirs {:?}, {:?}", a.path(), b.path())
            }
            Difference::FileType(a, b) => format!(
                "Different file types in files {:?}, {:?}",
                a.path(),
                b.path()
            ),
            Difference::FileName(a, b) => format!(
                "Different file names in files {:?}, {:?}",
                a.path(),
                b.path()
            ),
            Difference::FileContents(a, b) => format!(
                "Different file contents in files {:?}, {:?}:\n{}",
                a.path(),
                b.path(),
                format_file_diff(a.path(), b.path()).unwrap()
            ),
            Difference::LeftoverEntriesA(entry) => format!("in A but not in B: {:?}", entry.path()),
            Difference::LeftoverEntriesB(entry) => format!("in B but not in A: {:?}", entry.path()),
        };
        write!(f, "{}", s)
    }
}

pub fn format_file_diff(a: &Path, b: &Path) -> Result<String, Error> {
    let contents_a = fs::read_to_string(a)?;
    let contents_b = fs::read_to_string(b)?;
    println!("{}", contents_a);
    println!("{}", contents_b);
    let changes = diff(&contents_a, &contents_b);
    Ok(changes
        .iter()
        .map(|change| format!("{:?}", change))
        .join("\n"))
}

/// Are the contents of two directories different?
///
/// # Examples
///
/// ```no_run
/// extern crate dir_diff;
///
/// assert!(dir_diff::is_different("dir/a", "dir/b").unwrap());
/// ```
#[allow(dead_code)]
pub fn get_first_difference<A: AsRef<Path>, B: AsRef<Path>>(
    a_base: A,
    b_base: B,
) -> Result<Option<Difference>, Error> {
    let mut a_walker = walk_dir(a_base)?;
    let mut b_walker = walk_dir(b_base)?;

    for (a, b) in (&mut a_walker).zip(&mut b_walker) {
        let a = a?;
        let b = b?;

        if a.depth() != b.depth() {
            return Ok(Some(Difference::Depth(a, b)));
        }
        if a.file_type() != b.file_type() {
            return Ok(Some(Difference::FileType(a, b)));
        }
        if a.file_name() != b.file_name() {
            return Ok(Some(Difference::FileName(a, b)));
        }
        if a.file_type().is_file() && (read_to_vec(a.path())? != read_to_vec(b.path())?) {
            return Ok(Some(Difference::FileContents(a, b)));
        }
    }

    match a_walker.next() {
        Some(entry) => Ok(Some(Difference::LeftoverEntriesA(entry?))),
        None => match b_walker.next() {
            Some(entry) => Ok(Some(Difference::LeftoverEntriesB(entry?))),
            None => Ok(None),
        },
    }
}

#[allow(dead_code)]
pub fn is_different<A: AsRef<Path>, B: AsRef<Path>>(a_base: A, b_base: B) -> Result<bool, Error> {
    get_first_difference(a_base, b_base).map(|difference| !difference.is_none())
}

pub fn check_dir_diff(source: &Path, target: &Path) -> () {
    assert!(!(is_different(source, target).unwrap()));
}

fn walk_dir<P: AsRef<Path>>(path: P) -> Result<walkdir::IntoIter, std::io::Error> {
    let mut walkdir = WalkDir::new(path).sort_by(compare_by_file_name).into_iter();
    if let Some(Err(e)) = walkdir.next() {
        Err(e.into())
    } else {
        Ok(walkdir)
    }
}

fn compare_by_file_name(a: &DirEntry, b: &DirEntry) -> Ordering {
    a.file_name().cmp(b.file_name())
}

fn read_to_vec<P: AsRef<Path>>(file: P) -> Result<Vec<u8>, std::io::Error> {
    let mut data = Vec::new();
    let mut file = File::open(file.as_ref())?;

    file.read_to_end(&mut data)?;

    Ok(data)
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Error {
        Error::StripPrefix(e)
    }
}

impl From<walkdir::Error> for Error {
    fn from(e: walkdir::Error) -> Error {
        Error::WalkDir(e)
    }
}
