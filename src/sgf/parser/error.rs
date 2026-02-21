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
        write!(formatter, "Could not parse {} from {:?}: {}", self.token, self.input, self.reason)
    }
}

impl std::error::Error for Error { }

impl From<Error> for common::error::Error {
    fn from(error: Error) -> common::error::Error {
        common::error::Error::new(&error.to_string())
    }
}

impl From<nom::Error<&str>> for Error {
    fn from(nom_error: nom::Error<&str>) -> Self {
        Self{ 
            token: nom_error.context,
            input: nom_error.input,
            reason: nom_error.kind,
        }
    }
}

