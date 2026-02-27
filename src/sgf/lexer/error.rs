use std::fmt;

use crate::common;

#[derive(Debug, PartialEq)]
pub struct Error { 
    error: ErrorType,
}

#[derive(Debug, PartialEq)]
enum ErrorType {
    Char(char),
    Empty,
    Message(String),
}

impl Error {
    pub fn character(string: &str) -> Self {
        match string.chars().next() {
            Some(character) => Self{ error: ErrorType::Char(character) },
            None => Self{ error: ErrorType::Empty },  // TODO: add log
        }
    }

    pub fn empty() -> Self {
        Self{ error: ErrorType::Empty }
    }

    pub fn message(string: &str) -> Self {
        Self{ error: ErrorType::Message(string.to_string()) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.error {
            ErrorType::Char(character) => write!(formatter, "Could not lex '{}'", character),
            ErrorType::Empty => write!(formatter, "Could not lex from empty string"),
            ErrorType::Message(message) => write!(formatter, "Lexer failed with message: {}", message),
        }
    }
}

impl std::error::Error for Error { }

impl From<Error> for common::Error {
    fn from(error: Error) -> common::Error {
        common::Error::new(&error.to_string())
    }
}

