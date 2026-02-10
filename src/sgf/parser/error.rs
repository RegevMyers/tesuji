use std::fmt;

use crate::common;

#[derive(Debug)]
pub struct Error { }

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Yuh there was an error ...")
    }
}

impl std::error::Error for Error { }

impl From<Error> for common::error::Error{
    fn from(error: Error) -> common::error::Error {
        todo!()
    }
}

