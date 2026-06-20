use crate::common::prolog::*;

use crate::board::{Board, Intersection};

use colored::{ColoredString, Colorize};

use crate::common::SplitEnds;

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Note the Array2D and graphical notion of "row" and "col" are inverse.
        let rows: Vec<Vec<&Intersection>> = self.board.columns_iter().map(|row| row.collect()).collect();

        if let Some((top, mid, bottom)) = rows.as_slice().split_ends() {
            Self::print_row(top, Self::EMPTY_TOP_ROW, formatter)?;

            for row in mid {
                Self::print_row(row, Self::EMPTY_ROW, formatter)?;
            }

            Self::print_row(bottom, Self::EMPTY_BOTTOM_ROW, formatter)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
struct EmptySymbols {
    left: char,
    mid: char,
    right: char,
}

impl Board {
    fn print_row(row: &Vec<&Intersection>, empty_symbols: EmptySymbols, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, mid, last)) = row.as_slice().split_ends() {
            Self::print_intersection(first, empty_symbols.left, Self::CONNECTOR, formatter)?;

            for intersection in mid {
                Self::print_intersection(intersection, empty_symbols.mid, Self::CONNECTOR, formatter)?;
            }

            Self::print_intersection(last, empty_symbols.right, Self::NEW_LINE, formatter)?;
        }

        Ok(())
    }

    fn print_intersection(intersection: &Intersection, empty: char, connector: char, formatter: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match intersection.stone {
            Some(Color::Black) => ColoredString::from(Self::BLACK_CIRCLE.to_string()),
            Some(Color::White) => ColoredString::from(Self::WHITE_CIRCLE.to_string()),
            None if intersection.star => Self::STAR.to_string().dimmed().white(),
            None => empty.to_string().dimmed().white(),
        };

        write!(formatter, "{}{}", symbol, connector.to_string().dimmed().white())?;

        Ok(())
    }
}

#[rustfmt::skip]
impl Board {
    const BLACK_CIRCLE: char        = '\u{25EF}'; // ◯
    const WHITE_CIRCLE: char        = '\u{2B24}'; // ⬤

    const EMPTY_TOP_LEFT: char      = '\u{250C}'; // ┌
    const EMPTY_TOP: char           = '\u{252C}'; // ┬
    const EMPTY_TOP_RIGHT: char     = '\u{2510}'; // ┐

    const EMPTY_LEFT: char          = '\u{251C}'; // ├ 
    const EMPTY: char               = '\u{253C}'; // ┼
    const EMPTY_RIGHT: char         = '\u{2524}'; // ┤

    const EMPTY_BOTTOM_LEFT: char   = '\u{2514}'; // └
    const EMPTY_BOTTOM: char        = '\u{2534}'; // ┴
    const EMPTY_BOTTOM_RIGHT: char  = '\u{2518}'; // ┘

    const STAR: char                = '\u{256C}'; // ╬

    const CONNECTOR: char           = '\u{2500}'; // ─

    const NEW_LINE: char            = '\n';

    const EMPTY_TOP_ROW:    EmptySymbols = EmptySymbols { left: Self::EMPTY_TOP_LEFT,    mid: Self::EMPTY_TOP,    right: Self::EMPTY_TOP_RIGHT    };
    const EMPTY_ROW:        EmptySymbols = EmptySymbols { left: Self::EMPTY_LEFT,        mid: Self::EMPTY,        right: Self::EMPTY_RIGHT        };
    const EMPTY_BOTTOM_ROW: EmptySymbols = EmptySymbols { left: Self::EMPTY_BOTTOM_LEFT, mid: Self::EMPTY_BOTTOM, right: Self::EMPTY_BOTTOM_RIGHT };
}
