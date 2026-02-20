use std::fmt;

use crate::common;

#[derive(Debug)]
pub struct Error { 
    string: String,
    token: String,
    reason: String
}

impl Error {
    pub fn new(string: &str, token: &str, reason: &str) -> Self {
        Self{
            string: string.to_string(),
            token: token.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Could not parse {} from {:?}: {}", self.token, self.string, self.reason)
    }
}

impl std::error::Error for Error { }

impl From<Error> for common::error::Error {
    fn from(error: Error) -> common::error::Error {
        common::error::Error::new(error.to_string())
    }
}

