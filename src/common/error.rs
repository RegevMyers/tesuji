use crate::common::log;

use thiserror::Error;

use array2d;
use reqwest;
use sgf_parse;

use std::io;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("Array2D: {0}")]
    Array2D(#[from] array2d::Error),

    #[error("Sgf: {0}")]
    Sgf(#[from] sgf_parse::SgfParseError),

    #[error("IO: {0}")]
    Io(#[from] io::Error),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl Error {
    pub fn message(message: &str) -> Self {
        Self::Message(message.to_string())
    }

    pub fn log(&self) {
        log::fatal(&self.to_string())
    }
}
