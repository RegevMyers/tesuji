use crate::sgf::parser;

use std::num::{ ParseIntError, ParseFloatError };
use std::char::ParseCharError;

use nom::{
    Parser,
    IResult,
    combinator::{ recognize, map, map_res },
    character::complete::{ digit1 },
    bytes::complete::{ tag },
    branch::{ alt },
    error::{ Error, ErrorKind },
};

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

pub fn none(string: &str) -> IResult<&str, Value, parser::Error> {
    Ok((string, Value::None))
}

pub fn number(string: &str) -> IResult<&str, Value> {
    recognize(digit1).map_res(str::parse).map(Value::Number).parse(string)
}

pub fn real(string: &str) -> IResult<&str, Value> {
    recognize(double).map_res(str::parse).map(Value::Real).parse(string)
}

pub fn double(string: &str) -> IResult<&str, Value> {
    alt((
        tag("1").map(|_| Value::Double(Double::Once)),
        tag("2").map(|_| Value::Double(Double::Twice)),
    )).parse(string)
}

pub fn color(string: &str) -> Result<Value, parser::Error> {
    alt((
        tag("B").map(|_| Value::Color(Color::Black)),
        tag("W").map(|_| Value::Color(Color::White)),
    )).parse(string)
}

pub fn simple_text(string: &str) -> Result<Value, parser::Error> {
    todo!()
}

pub fn text(string: &str) -> Result<Value, parser::Error> {
    todo!()
}

pub fn r#move(string: &str) -> Result<Value, parser::Error> {
    match string.len() {
        0 => Ok(Value::Move(Move::Pass)),
        2 => Ok(Value::Move(stone(string)?)),
        _ => Err(parser::Error::new("Value::Move", string, "length must be 2 (for a move) or 0 (for a pass)")),
    }
}

fn stone(string: &str) -> Result<Move, parser::Error> {
    let (x, y) = string.split_at(1);
    Ok(Move::Stone{ x: line(x)?, y: line(y)? })
}

fn line(string: &str) -> Result<u8, parser::Error> {
    match string.parse().map_err(|e: ParseCharError| parser::Error::new("char", string, &e.to_string()))? {
        lowercase if ('a' <= lowercase && lowercase <= 'z') => Ok((lowercase as u8) - ('a' as u8)),
        uppercase if ('A' <= uppercase && uppercase <= 'Z') => Ok((uppercase as u8) - ('A' as u8)),
        _ => Err(parser::Error::new("line", string, "line must be a letter ([a-z] or [A-Z])"))
    }
}

pub fn compose(parse_a: Parser, parse_b: Parser, string: &str) -> Result<Value, parser::Error> {
    let (a_str, b_str) = string.split_once(':').ok_or(parser::Error::new("Value::Compose", string, "no compose separator (':') found"))?;

    let a = parse_a(a_str)?;
    let b = parse_b(b_str)?;

    Ok(Value::Compose(Box::new(a), Box::new(b)))
}

