use std::fmt;

use crate::common;

pub struct Error { 
    character: char,
}

impl Error {
    pub fn new(character: char) -> Self {
        Self{ character }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Could not lex '{}'", self.character)
    }
}

impl std::error::Error for Error { }

impl From<Error> for common::Error {
    fn from(error: Error) -> common::Error {
        common::Error::new(&error.to_string())
    }
}

