use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct NoteNotInNoteFolderError {
    pub path: PathBuf,
}

impl Error for NoteNotInNoteFolderError {}

impl fmt::Display for NoteNotInNoteFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Path is not in the note folder: {}",
            self.path.to_str().unwrap()
        )
    }
}

#[derive(Debug)]
pub struct InvalidNameError {
    pub name: String,
}

impl Error for InvalidNameError {}

impl fmt::Display for InvalidNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name is not valid: {}", self.name)
    }
}
