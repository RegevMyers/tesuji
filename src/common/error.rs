use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for Error { }

