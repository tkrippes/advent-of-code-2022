use std::{error, fmt, io};

#[derive(Debug, PartialEq)]
pub enum Error {
    IOError { cause: String },
    ParsingAssignmentPairsError { line_index: usize, cause: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOError { cause } => write!(f, "parsing error, IO error, {}", cause),
            Error::ParsingAssignmentPairsError { line_index, cause } => {
                write!(f, "parsing error at line {}, {}", line_index, cause)
            }
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError {
            cause: err.to_string(),
        }
    }
}

impl From<(usize, String)> for Error {
    fn from(err: (usize, String)) -> Self {
        Error::ParsingAssignmentPairsError {
            line_index: err.0,
            cause: err.1,
        }
    }
}
