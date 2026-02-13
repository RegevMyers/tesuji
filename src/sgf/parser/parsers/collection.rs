use std::str::FromStr;

use crate::sgf::parser::error::Error;

#[derive(Debug)]
pub struct Collection {
    firstchar: char
}

impl FromStr for Collection {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Collection{ firstchar: string.chars().next().unwrap() })  // RHHAAAA !!!
    }
}

