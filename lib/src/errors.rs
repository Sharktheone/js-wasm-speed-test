use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TestError {
    InvalidFileType,
    Other(Box<dyn Error>),
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::InvalidFileType => write!(f, "Invalid file type"),
            TestError::Other(err) => write!(f, "{}", err),
        }
    }
}

impl Error for TestError {}
