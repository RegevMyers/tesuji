use crate::sgf::parser;

pub trait Parser<T> { }

impl<T, F> Parser<T> for F where F: Fn(&str) -> Result<(T, String), parser::Error> { }

