pub enum Value {
    None,
    Number(i64),
    Real(f64),
    Double(Double),
    Color(Color),
    SimpleText,
    Text,
    Move(Move),
}

pub enum Double {
    Once,
    Twice,
}

// TODO: Move to common?
pub enum Color {
    Black,
    White,
}

pub enum Move {
    Stone{ x: u8, y: u8 },
    Pass,
}

pub fn none(string: &str) -> Option<Value> {
    match string { 
        "" => Some(Value::None),
        _  => None,
    }
}

pub fn number(string: &str) -> Option<Value> {
    Some(Value::Number(string.parse().ok()?))
}

pub fn real(string: &str) -> Option<Value> {
    Some(Value::Real(string.parse().ok()?))
}

pub fn double(string: &str) -> Option<Value> {
    match string {
        "1" => Some(Value::Double(Double::Once)),
        "2" => Some(Value::Double(Double::Twice)),
        _   => None,
    }
}

pub fn color(string: &str) -> Option<Value> {
    match string {
        "B" => Some(Value::Color(Color::Black)),
        "W" => Some(Value::Color(Color::White)),
        _   => None,
    }
}

pub fn simple_text(string: &str) -> Option<Value> {
    todo!()
}

pub fn text(string: &str) -> Option<Value> {
    todo!()
}

pub fn r#move(string: &str) -> Option<Value> {
    match string.len() {
        0 => Some(Value::Move(Move::Pass)),
        2 => Some(Value::Move(stone(string)?)),
        _ => None,
    }
}

fn stone(string: &str) -> Option<Move> {
    let (x, y) = string.split_at(1);
    Some(Move::Stone{ x: line(x)?, y: line(y)? })
}

fn line(string: &str) -> Option<u8> {
    match string.parse().ok()? {
        lowercase if ('a' <= lowercase && lowercase <= 'z') => Some((lowercase as u8) - ('a' as u8)),
        uppercase if ('A' <= uppercase && uppercase <= 'Z') => Some((uppercase as u8) - ('A' as u8)),
        _ => None
    }
}

