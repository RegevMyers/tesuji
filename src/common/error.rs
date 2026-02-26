use crate::log;

use std::fmt;

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

