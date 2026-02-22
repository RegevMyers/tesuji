use std::fmt;

use crate::common;

#[derive(Debug)]
pub struct Error { 
    token: String,
    input: String,
    reason: String
}

impl Error {
    pub fn new(token: &str, input: &str, reason: &str) -> Self {
        Self{
            token: token.to_string(),
            input: input.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Could not parse token {} from {:?}: {}", self.token, self.input, self.reason)
    }
}

impl std::error::Error for Error { }

// TODO: this should be in common::Error, not here
impl From<Error> for common::Error {
    fn from(error: Error) -> common::Error {
        common::Error::new(&error.to_string())
    }
}

