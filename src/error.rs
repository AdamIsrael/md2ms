use std::fmt;

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
