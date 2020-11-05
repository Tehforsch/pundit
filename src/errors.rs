use std::fmt;
use std::path::{PathBuf};
use std::error::Error;

#[derive(Debug)]
pub struct NoteNotInNoteFolderError {
    pub path: PathBuf
}

impl Error for NoteNotInNoteFolderError {
    
}

impl fmt::Display for NoteNotInNoteFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path is not in the note folder: {}", self.path.to_str().unwrap())
    }
}
