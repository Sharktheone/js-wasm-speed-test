use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum TestError {
    IsDir,
    IsFile,
    InvalidFileType,
    AlreadyInitialized,
    String(String),
    Other(Box<dyn Error>),
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::InvalidFileType => write!(f, "Invalid file type"),
            TestError::IsDir => write!(f, "Path is a directory"),
            TestError::IsFile => write!(f, "Path is a file"),
            TestError::AlreadyInitialized => write!(f, "V8 is already initialized"),
            TestError::Other(err) => write!(f, "{}", err),
            TestError::String(err) => write!(f, "{}", err),
            _ => write!(f, "Unknown error"),
        }
    }
}



impl From<IoError> for TestError {
    fn from(err: IoError) -> Self {
        TestError::Other(Box::new(err))
    }
}


impl Error for TestError {}