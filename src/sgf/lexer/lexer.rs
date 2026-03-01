use crate::sgf::lexer;

use crate::common::Color;
use crate::common::utils;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Double {
    Once,
    Twice,
}

#[derive(Debug, PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub fn number(input: &str) -> Result<(Token, &str), lexer::Error> {
    let (number, rest) = take_while(|c: char| c.is_ascii_digit(), input)?;
    Ok((Token::Number(number.to_string()), rest))
}

pub fn real(input: &str) -> Result<(Token, &str), lexer::Error> {
    let (left, rest) = take_while(|c: char| c.is_ascii_digit(), input)?;
    let (dot, rest) = take_one_of('.', rest)?;
    let (right, rest) = take_while(|c: char| c.is_ascii_digit(), rest)?;

    Ok((Token::Real(format!("{}{}{}", left, dot, right)), rest))
}

pub fn double(input: &str) -> Result<(Token, &str), lexer::Error> {
    match take_one(input) {
        Ok(("1", rest)) => Ok((Token::Double(Double::Once), rest)),
        Ok(("2", rest)) => Ok((Token::Double(Double::Twice), rest)),
        _ => Err(lexer::Error::message("Token::Double must be '1' or '2'")),
    }
}

pub fn color(input: &str) -> Result<(Token, &str), lexer::Error> {
    match take_one(input) {
        Ok(("B", rest)) => Ok((Token::Color(Color::Black), rest)),
        Ok(("W", rest)) => Ok((Token::Color(Color::White), rest)),
        _ => Err(lexer::Error::message("Token::Color must be 'B' or 'W'")),
    }
}

pub fn simple_text(input: &str) -> Result<(Token, &str), lexer::Error> {
    todo!()
}

pub fn text(input: &str) -> Result<(Token, &str), lexer::Error> {
    todo!()
}

pub fn stone(input: &str) -> Result<(Token, &str), lexer::Error> {
    let (x, rest) = take_one_if(|c: char| c.is_ascii_alphabetic(), input)?;
    let (y, rest) = take_one_if(|c: char| c.is_ascii_alphabetic(), rest)?;

    Ok((Token::Stone(format!("{}{}", x, y)), rest))
}

pub fn identifier(input: &str) -> Result<(Token, &str), lexer::Error> {
    let (identifier, rest) = take_while(|c: char| c.is_uppercase(), input)?;

    //if identifier.length > 2 { log::hint(format!("Lexed a non-standard identifier: '{}'. Identifier length is more than two characters"), identifier); }
    
    Ok((Token::Identifier(identifier.to_string()), rest))
}

pub fn paren(input: &str) -> Result<(Token, &str), lexer::Error> {
    match take_one(input) {
        Ok(("(", rest)) => Ok((Token::Paren(Side::Left), rest)),
        Ok((")", rest)) => Ok((Token::Paren(Side::Right), rest)),
        _ => Err(lexer::Error::message("Token::Paren must be '(' or ')'")),
    }
}

pub fn square_paren(input: &str) -> Result<(Token, &str), lexer::Error> {
    // TODO: make a function so its like :
    //  
    //  match_char(
    //      ("[", Token::SquareParen(Side::Left)),
    //      ("]", Token::SquareParen(Side::Right)),
    //  )
    match take_one(input) {
        Ok(("[", rest)) => Ok((Token::SquareParen(Side::Left), rest)),
        Ok(("]", rest)) => Ok((Token::SquareParen(Side::Right), rest)),
        _ => Err(lexer::Error::message("Token::SquareParen must be '[' or ']'")),
    }
}

pub fn colon(input: &str) -> Result<(Token, &str), lexer::Error> {
    take_one_of(':', input).map(|(_colon, rest)| Ok((Token::Colon, rest)))?
}

pub fn semicolon(input: &str) -> Result<(Token, &str), lexer::Error> {
    take_one_of(';', input).map(|(_semicolon, rest)| Ok((Token::Semicolon, rest)))?
}

fn take_while(predicate: impl Fn(char) -> bool, input: &str) -> Result<(&str, &str), lexer::Error> {
    if input.is_empty() {
        return Err(lexer::Error::empty())
    }

    let position = input.find(|character: char| !predicate(character));
    match position {
        Some(0) => Err(lexer::Error::character(input)),
        Some(position) => Ok((&input[..position], &input[position..])),
        None => Ok((input, ""))
    }
}

fn take_one(input: &str) -> Result<(&str, &str), lexer::Error> {
    take_one_if(|_: char| true, input)
}

fn take_one_of(character: char, input: &str) -> Result<(&str, &str), lexer::Error> {
    take_one_if(|c: char| c == character, input)
}

fn take_one_if(predicate: impl Fn(char) -> bool, input: &str) -> Result<(&str, &str), lexer::Error> {
    match input.chars().next() {
        Some(c) if predicate(c) => Ok((&input[..1], &input[1..])),
        _ => Err(lexer::Error::character(input)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn number() {
        assert_eq!(super::number("1337a"), Ok((super::Token::Number("1337".to_string()), "a")))
    }

    #[test]
    pub fn real() {
        assert_eq!(super::real("67.67"), Ok((super::Token::Real("67.67".to_string()), "")));
        assert!(super::real("67").is_err());
        assert!(super::real(".67").is_err());
        assert!(super::real("67.").is_err());
    }

    #[test]
    pub fn double() {
        assert_eq!(super::double("1"), Ok((super::Token::Double(super::Double::Once), "")));
        assert_eq!(super::double("2a"), Ok((super::Token::Double(super::Double::Twice), "a")));
    }

    #[test]
    pub fn color() {
        assert_eq!(super::color("B"), Ok((super::Token::Color(super::Color::Black), "")));
        assert_eq!(super::color("Wa"), Ok((super::Token::Color(super::Color::White), "a")));
    }

    #[test]
    #[ignore = "todo"]
    pub fn simple_text() {
        todo!()
    }

    #[test]
    #[ignore = "todo"]
    pub fn text() {
        todo!()
    }

    #[test]
    pub fn stone() {
        assert!(super::stone("a").is_err());
        assert_eq!(super::stone("aa"), Ok((super::Token::Stone("aa".to_string()), "")));
        assert_eq!(super::stone("AZ"), Ok((super::Token::Stone("AZ".to_string()), "")));
        assert_eq!(super::stone("abc"), Ok((super::Token::Stone("ab".to_string()), "c")));
        assert!(super::stone("1abc").is_err());
    }

    #[test]
    pub fn identifier() {
        assert_eq!(super::identifier("EZ"), Ok((super::Token::Identifier("EZ".to_string()), "")));
    }

    #[test]
    pub fn paren() {
        assert_eq!(super::paren("("), Ok((super::Token::Paren(super::Side::Left), "")));
        assert_eq!(super::paren(")a"), Ok((super::Token::Paren(super::Side::Right), "a")));
    }

    #[test]
    pub fn square_paren() {
        assert_eq!(super::square_paren("["), Ok((super::Token::SquareParen(super::Side::Left), "")));
        assert_eq!(super::square_paren("]"), Ok((super::Token::SquareParen(super::Side::Right), "")));
    }

    #[test]
    pub fn colon() {
        assert_eq!(super::colon(":"), Ok((super::Token::Colon, "")));
    }

    #[test]
    pub fn semicolon() {
        assert_eq!(super::semicolon(";"), Ok((super::Token::Semicolon, "")));
    }
}

