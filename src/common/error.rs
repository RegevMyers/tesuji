use thiserror::Error;

use crate::common::log;

use sgf_parse::SgfParseError;
use std::io;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    Sgf(#[from] SgfParseError),

    #[error(transparent)]
    Io(#[from] io::Error),
}

impl Error {
    pub fn message(message: &str) -> Self {
        Self::Message(message.to_string())
    }

    pub fn log(&self) {
        log::fatal(&self.to_string())
    }
}
