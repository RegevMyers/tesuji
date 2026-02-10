use std::fmt;

#[derive(Debug)]
pub struct Error { }

impl fmt::Display for Error {
     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}

