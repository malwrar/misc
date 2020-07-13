//! Error and Result module.
use std::fmt;
use std::result::Result as StdResult;

/// Result returned my methods that can fail.
pub type Result<T> = StdResult<T, Error>;

pub struct Error {
    pub reason: &'static str
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error {
    pub fn new(reason: &'static str) -> Error {
        Error { reason: reason }
    }
}
