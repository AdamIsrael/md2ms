use std::fmt;
use thiserror::Error;
// use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum Md2msError {
    #[error("Included file not found")]
    FileNotFound(String),
    #[error("Metadata is missing `include` key")]
    NoFilesInMetadata,
    #[error("Error packing the document")]
    PackError,
    // PackError(#[from] ZipError),
    #[error("An unknown error occurred")]
    Unknown,
}

#[derive(Debug)]
pub enum ObsidianError {
    DirectoryCreationError,
    HttpError,
    OtherError,
    ParseError,
}

impl fmt::Display for ObsidianError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObsidianError::DirectoryCreationError => {
                write!(f, "Couldn't create ~/.md2ms/obsidian directory")
            }
            ObsidianError::HttpError => write!(f, "HTTP Error"),
            ObsidianError::OtherError => write!(f, "Other Error"),
            ObsidianError::ParseError => write!(f, "Parse Error"),
        }
    }
}
