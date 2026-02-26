use crate::sgf::parser;

use crate::common::Error;
use crate::common::logging as log;

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

// TODO: Move to common?
#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub enum Move {
    Stone{ x: u8, y: u8 },
    Pass,
}

pub fn none(string: &str) -> Result<(Value, &str), parser::Error> {
    Ok((Value::None, string))
}

pub fn number(string: &str) -> Result<(Value, &str), parser::Error> {
    let is_non_digit = |c: &char| !c.is_ascii_digit();
    
    match string.chars().find(is_non_digit) {
        Some(end) => Ok((Value::Number(string[..end].parse()?), string[end..])),
        None => Err(parser::Error::new("Value::Number", string, "No digits found"))
    }
}

pub fn real(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!() //nom_double.map(Value::Real).parse(string)
}

pub fn double(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!() //alt((
        //value(Value::Double(Double::Once), tag("1")),
        //value(Value::Double(Double::Twice), tag("2")),
    //)).parse(string)
}

pub fn color(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!() //alt((
        //value(Value::Color(Color::Black), tag("B")),
        //value(Value::Color(Color::White), tag("W")),
    //)).parse(string)
}

pub fn simple_text(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn text(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
}

pub fn r#move(string: &str) -> Result<(Value, &str), parser::Error> {
    todo!()
        //alt((
        //value(Value::Move(Move::Pass), none),
        //stone.map(Value::Move),
    //)).parse(string)
}

fn stone(string: &str) -> Result<(Move, &str), parser::Error> {
    todo!() //(line, line).map(|(x, y)| Move::Stone{x, y}).parse(string)
}

fn line(string: &str) -> Result<(u8, &str), parser::Error> {
    todo!()
    //alt((
        //satisfy(|c: char| c.is_ascii_lowercase()).map(|c: char| (c as u8) - ('a' as u8)),
        //satisfy(|c: char| c.is_ascii_uppercase()).map(|c: char| (c as u8) - ('A' as u8)),
    //)).parse(string)
}

pub fn compose(parse_a: impl parser::Parser<Value>, parse_b: impl parser::Parser<Value>, string: &str) -> Result<(Value, &str), parser::Error> {
    todo!() //(parse_a, tag(":"), parse_b).map(|(a, _, b)|
        //Value::Compose(Box::new(a), Box::new(b))
    //).parse(string)
}

#[cfg(test)]
mod test {
    #[test]
    fn none() -> Result<(), super::Error> {
        Ok(())

    }
}
