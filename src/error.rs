use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

#[derive(Debug)]
pub enum ProcError {
    Io(io::Error),
    Utf8(Utf8Error),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    UnexpectedFormat(String),
    ParseError,
    InternalError,
    PermissionDenied,
    NotFound,
}

impl fmt::Display for ProcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcError::Io(e) => write!(f, "IO error: {}", e),
            ProcError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            ProcError::ParseInt(e) => write!(f, "Parse int error: {}", e),
            ProcError::ParseFloat(e) => write!(f, "Parse float error: {}", e),
            ProcError::UnexpectedFormat(s) => write!(f, "Unexpected format: {}", s),
            ProcError::ParseError => write!(f, "Parse error"),
            ProcError::InternalError => write!(f, "Internal error"),
            ProcError::PermissionDenied => write!(f, "Permission denied"),
            ProcError::NotFound => write!(f, "Not found"),
        }
    }
}

impl std::error::Error for ProcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcError::Io(e) => Some(e),
            ProcError::Utf8(e) => Some(e),
            ProcError::ParseInt(e) => Some(e),
            ProcError::ParseFloat(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ProcError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::PermissionDenied => ProcError::PermissionDenied,
            io::ErrorKind::NotFound => ProcError::NotFound,
            _ => ProcError::Io(err),
        }
    }
}

impl From<Utf8Error> for ProcError {
    fn from(err: Utf8Error) -> Self {
        ProcError::Utf8(err)
    }
}

impl From<ParseIntError> for ProcError {
    fn from(err: ParseIntError) -> Self {
        ProcError::ParseInt(err)
    }
}

impl From<ParseFloatError> for ProcError {
    fn from(err: ParseFloatError) -> Self {
        ProcError::ParseFloat(err)
    }
}

pub type ProcResult<T> = Result<T, ProcError>;
