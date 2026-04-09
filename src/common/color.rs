use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
