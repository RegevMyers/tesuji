use crate::sgf::parser;

use crate::common::{
    Color,
    Error,
};

use crate::common::log;

#[derive(Debug)]
pub enum Value {
    None,
    Number(i64),
    Real(f64),
    Double(Double),
    Color(Color),
    SimpleText,
    Text,
    Move(Move),
    Compose(Box<Value>, Box<Value>)
}

#[derive(Debug)]
pub enum Double {
    Once,
    Twice,
}

#[derive(Debug)]
pub enum Move {
    Stone{ x: u8, y: u8 },
    Pass,
}

pub fn none(string: &str) -> Result<(Value, &str), parser::Error> {
    let (number, rest) = take_while(char::is_ascii_digit, input)?;
    Ok((Token::Number(number.to_string()), rest))
}

pub fn number(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn real(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn double(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn color(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn simple_text(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn text(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn r#move(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

fn stone(string: &str) -> Result<(Move, &str), parser::Error> {
    todo!()
}

fn line(string: &str) -> Result<(u8, &str), parser::Error> {
    todo!()
}

fn take_while(predicate: impl Fn(char) -> bool, input: &str) -> Result<(&str, &str), Option<char>> {
    if input.is_empty() {
        return Err(parser::Error::empty())
    }

    let position = input.find(|character: char| !predicate(character)).ok_or(input.len() - 1);

    match position {
        0 => Err(Some(input.chars().next())),
        position => Ok((&input[..position], &input[position..])),
    }
}

fn take_one(input: &str) -> Result<(&str, &str), Option<char>> {
    take_one_if(|_: char| true, input)
}

fn take_one_of(character: char, input: &str) -> Result<(&str, &str), Option<char>> {
    take_one_if(|c: char| c == character, input)
}

fn take_one_if(predicate: impl Fn(char) -> bool, input: &str) -> Result<(&str, &str), Option<char>> {
    match input.chars().next() {
        Some(c) if predicate(c) => Ok((&input[..1], &input[1..])),
        Some(c) => Err(Some(c)),
        None => Err(None)
    }
}

//pub fn compose(parse_a: impl parser::Parser<Value>, parse_b: impl parser::Parser<Value>, string: &str) -> Result<(Value, &str), parser::Error> {
//    todo!()
//}

#[cfg(test)]
mod test {
    #[test]
    fn none() -> Result<(), super::Error> {
        Ok(())

    }
}
