use crate::sgf::parser;

use nom::{
    Parser,
    IResult,
    branch::{ alt },
    combinator::{ value },
    bytes::complete::{ tag },
    number::complete::{ double as nom_double },
    character::complete::{ i64 as nom_i64, satisfy },
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Double {
    Once,
    Twice,
}

// TODO: Move to common?
#[derive(Debug, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub enum Move {
    Stone{ x: u8, y: u8 },
    Pass,
}

pub type ValueParser = fn(&str) -> IResult<&str, Value>;

pub fn none(string: &str) -> IResult<&str, Value> {
    Ok((string, Value::None))
}

pub fn number(string: &str) -> IResult<&str, Value> {
    nom_i64.map(Value::Number).parse(string)
}

pub fn real(string: &str) -> IResult<&str, Value> {
    nom_double.map(Value::Real).parse(string)
}

pub fn double(string: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Double(Double::Once), tag("1")),
        value(Value::Double(Double::Twice), tag("2")),
    )).parse(string)
}

pub fn color(string: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Color(Color::Black), tag("B")),
        value(Value::Color(Color::White), tag("W")),
    )).parse(string)
}

pub fn simple_text(string: &str) -> Result<Value, parser::Error> {
    todo!()
}

pub fn text(string: &str) -> Result<Value, parser::Error> {
    todo!()
}

pub fn r#move(string: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Move(Move::Pass), none),
        stone.map(Value::Move),
    )).parse(string)
}

fn stone(string: &str) -> IResult<&str, Move> {
    (line, line).map(|(x, y)| Move::Stone{x, y}).parse(string)
}

fn line(string: &str) -> IResult<&str, u8> {
    alt((
        satisfy(|c: char| c.is_ascii_lowercase()).map(|c: char| (c as u8) - ('a' as u8)),
        satisfy(|c: char| c.is_ascii_uppercase()).map(|c: char| (c as u8) - ('A' as u8)),
    )).parse(string)
}

pub fn compose(parse_a: ValueParser, parse_b: ValueParser, string: &str) -> IResult<&str, Value> {
    (parse_a, tag(":"), parse_b).map(|(a, _, b)| Value::Compose(Box::new(a), Box::new(b))).parse(string)
}

