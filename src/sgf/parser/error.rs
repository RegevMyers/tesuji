use std::fmt;

use crate::common;

#[derive(Debug)]
pub struct Error { 
    token: String,
    reason: String
}

impl Error {
    pub fn new(token: &str, reason: &str) -> Self {
        Self{
            token: token.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Could not parse {}: {}", self.token, self.reason)
    }
}

impl std::error::Error for Error { }

// TODO: this should be in common::Error, not here
impl From<Error> for common::Error {
    fn from(error: Error) -> common::Error {
        common::Error::new(&error.to_string())
    }
}

