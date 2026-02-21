use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self{ message: message.to_string() }
    }
}

impl fmt::Display for Error {
     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for Error { }

impl From<nom::error::Error<&str>> for Error {
    fn from(nom_error: nom::error::Error<&str>) -> Error {
        Error::new(&nom_error.to_string())
    }
}

