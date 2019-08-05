use std::fmt;

#[derive(Debug)]
pub enum FileErrors {    
    UnableToReadFile,
    UnableToParseFile
}

impl std::fmt::Display for FileErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {            
            FileErrors::UnableToReadFile => "Unable to read from file.",
            FileErrors::UnableToParseFile => "Unable to parse contents of file."
        })
    }
}