use crate::sgf::parser;

use crate::common::Error;
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
    todo!()
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
