use crate::sgf::parser;

use std::num::{ ParseIntError, ParseFloatError };
use std::char::ParseCharError;

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

pub fn none(string: &str) -> Result<Value, parser::Error> {
    match string { 
        "" => Ok(Value::None),
        _  => Err(parser::Error::new("Value::None", string, "String is not empty")),
    }
}

pub fn number(string: &str) -> Result<Value, parser::Error> {
    Ok(Value::Number(string.parse().map_err(|e: ParseIntError| parser::Error::new("i64", string, &e.to_string()))?))
}

pub fn real(string: &str) -> Result<Value, parser::Error> {
    Ok(Value::Real(string.parse().map_err(|e: ParseFloatError| parser::Error::new("f64", string, &e.to_string()))?))
}

pub fn double(string: &str) -> Result<Value, parser::Error> {
    match string {
        "1" => Ok(Value::Double(Double::Once)),
        "2" => Ok(Value::Double(Double::Twice)),
        _   => Err(parser::Error::new("Value::Double", string, "Must be '1' or '2'")),
    }
}

pub fn color(string: &str) -> Result<Value, parser::Error> {
    match string {
        "B" => Ok(Value::Color(Color::Black)),
        "W" => Ok(Value::Color(Color::White)),
        _   => Err(parser::Error::new("Value::Color", string, "Must be 'B' or 'W'")),
    }
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
        _ => Err(parser::Error::new("Value::Move", string, "Length must be 2 (for a move) or 0 (for a pass)")),
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
        _ => Err(parser::Error::new("line", string, "Line must be a letter ([a-z] or [A-Z])"))
    }
}

type Parser = fn(&str) -> Result<Value, parser::Error>;
pub fn compose(parse_a: Parser, parse_b: Parser, string: &str) -> Result<Value, parser::Error> {
    let (a_str, b_str) = string.split_once(':').ok_or(parser::Error::new(
        "Value::Compose",
        string,
        "No ':' found"
    ))?;

    let a = parse_a(a_str)?;
    let b = parse_b(b_str)?;

    Ok(Value::Compose(Box::new(a), Box::new(b)))
}

