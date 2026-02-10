use std::fmt;

#[derive(Debug)]
pub struct Error { 
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Yuh there was an error ...")
    }
}

impl std::error::Error for Error { }
