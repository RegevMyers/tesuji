use nom::{
    error
};

use nom::{
    Parser,
    error::{ ParseError, ErrorKind },
};

use std::fmt;

use crate::common;

#[derive(Debug)]
pub struct Error { 
    token: Option<String>,
    input: String,
    reason: String
}

impl Error {
    pub fn new(token: &str, input: &str, reason: &str) -> Self {
        Self{
            token: Some(token.to_string()),
            input: input.to_string(),
            reason: reason.to_string(),
        }
    }

    fn add_token(mut self, token: &str) {
        self.token = Some(token.to_string());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(token) = &self.token {
            write!(formatter, "Could not parse token {} from {:?}: {}", token, self.input, self.reason)
        }
        else {
            write!(formatter, "Could not parse {:?}: {}", self.input, self.reason)
        }
    }
}

impl std::error::Error for Error { }

// TODO: this should be in common::Error, not here
impl From<Error> for common::Error {
    fn from(error: Error) -> common::Error {
        common::Error::new(&error.to_string())
    }
}

impl ParseError<&str> for Error {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        Self{
            input: input.to_string(),
            token: None,
            reason: format!("Nom error with code: {:?}", kind),
        }
    }
    
    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

pub trait WithToken<I, O>: Parser<I, O, Error> + Sized {
    fn with_token(self, token: &str) -> impl Parser<I, O, Error> {
        self.map_err(|e: Self| e.add_token(token));
    }
}

impl<I, O, P> WithToken<I, O> for P where P: Parser<I, O, Error> { }

