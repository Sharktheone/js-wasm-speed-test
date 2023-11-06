use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TestError {
    IsDir,
    IsFile,
    InvalidFileType,
    AlreadyInitialized,
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
            _ => write!(f, "Unknown error"),
        }
    }
}

impl From<Box<dyn Error + 'static>> for TestError {
    fn from(err: Box<dyn Error>) -> Self {
        TestError::Other(err)
    }
}

impl From<std::io::Error> for TestError {
    fn from(err: std::io::Error) -> Self {
        TestError::Other(Box::new(err))
    }
}

impl Error for TestError {}
