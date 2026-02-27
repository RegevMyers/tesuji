use crate::common::Color;

pub enum Token { 
    Number(String),
    Real(String),
    Double(Double),
    Color(Color),
    SimpleText(String),
    Text(String),
    Stone(String),
    Identifier(String),
    Paren(Side),
    SquareParen(Side),
    Colon,
    Semicolon,
}

pub enum Double {
    Once,
    Twice,
}

pub enum Side {
    Left,
    Right,
}

pub trait Lexer<T> { }
impl<F> Lexer for F where F: Fn(&str) -> Result<(Token, &str), parser::Error> { }

pub fn number(string: &str) -> Result<(Token, &str), lexer::Error> {
    
}

fn take(predicate: impl Fn(char) -> bool, string: &str) -> Option<(&str, &str)> {
    let position = string.find(|character: char| !predicate(character))?;
    Some((string[..position], string[position..]))
}

