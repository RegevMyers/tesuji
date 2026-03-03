use sgf_parse::SgfParseError;

use crate::log;

use std::fmt;
use std::io;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self{ message: message.to_string() }
    }

    pub fn log(&self) {
        log::error(&self.to_string())
    }
}

impl fmt::Display for Error {
     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for Error { }

impl From<SgfParseError> for Error {
    fn from(sgf_parse_error: SgfParseError) -> Self {
        Self{ message: sgf_parse_error.to_string() }
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self{ message: io_error.to_string() }
    }
}
