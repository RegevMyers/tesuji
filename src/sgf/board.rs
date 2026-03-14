use array2d::Array2D;
use sgf_parse::go::{Move, Point, Prop};
use sgf_parse::{PropertyType, SgfNode, SgfProp};

use crate::common::{Color, Error, SplitEnds, log};

use std::collections::HashSet;
use std::fmt;

type Intersection = Option<Color>;

pub struct Board {
    board: Array2D<Intersection>,
}

#[derive(Copy, Clone)]
struct EmptySymbols {
    left: char,
    mid: char,
    right: char,
}

impl Board {
    pub fn new(nodes: Vec<&GoNode>) -> Result<Self, Error> {
        let root = nodes.first().ok_or(Error::message("No nodes"))?;

        let (board_x, board_y) = Self::get_board_dimensions(root)?;
        let mut board = Array2D::filled_with(None, board_x, board_y);

        log::info(&format!("Board | Dimensions: {}-{}", board_x, board_y));

        for node in nodes {
            let r#move = Self::get_move(node, (board_x, board_y));
            let setup_moves = Self::get_setups(node);

            log::trace(&format!("Node | Move: {:?}, Setup: {:?}", r#move, setup_moves));

            if let Some((color, Move::Move(Point { x, y }))) = r#move {
                board[(x.into(), y.into())] = Some(color);
            }

            for (color, points) in setup_moves {
                for Point { x, y } in points {
                    board[(x.into(), y.into())] = color;
                }
            }
        }

        Ok(Self { board })
    }
}

type GoNode = SgfNode<Prop>;

impl Board {
    fn get_board_dimensions(root: &GoNode) -> Result<(usize, usize), Error> {
        if !root.is_root {
            return Err(Error::message("Node is not root node"));
        }

        match root.get_property("SZ") {
            Some(&Prop::SZ((x, y))) => Ok((x.into(), y.into())),
            _ => Err(Error::message("Missing property in root node: SZ")),
        }
    }

    fn get_move(node: &GoNode, board_dimensions: (usize, usize)) -> Option<(Color, Move)> {
        let is_normal_board_size = {
            let (x, y) = board_dimensions;
            x <= 19 && y <= 19
        };

        match *(node.get_move()?) {
            Prop::B(Move::Move(Point { x: 19, y: 19 })) if is_normal_board_size => Some((Color::Black, Move::Pass)),
            Prop::W(Move::Move(Point { x: 19, y: 19 })) if is_normal_board_size => Some((Color::White, Move::Pass)),
            Prop::B(r#move) => Some((Color::Black, r#move)),
            Prop::W(r#move) => Some((Color::White, r#move)),
            _ => None,
        }
    }

    fn get_setups(node: &GoNode) -> Vec<(Option<Color>, HashSet<Point>)> {
        let setup_properties = node.properties().find(|prop| prop.property_type() == Some(PropertyType::Setup));

        setup_properties
            .into_iter()
            .filter_map(|prop| match prop {
                Prop::AB(points) => Some((Some(Color::Black), points.clone())),
                Prop::AW(points) => Some((Some(Color::White), points.clone())),
                Prop::AE(points) => Some((None, points.clone())),
                _ => None,
            })
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Note that the Array2D and graphical notion of "row" and "col" are inverse.
        let rows: Vec<Vec<&Option<Color>>> = self.board.columns_iter().map(|row| row.collect()).collect();

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

impl Board {
    fn print_row(row: &Vec<&Intersection>, empty_symbols: EmptySymbols, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, mid, last)) = row.as_slice().split_ends() {
            write!(formatter, "{}{}", Self::display_intersection(first, empty_symbols.left), Self::CONNECTOR)?;

            for intersection in mid {
                write!(formatter, "{}{}", Self::display_intersection(intersection, empty_symbols.mid), Self::CONNECTOR)?;
            }

            writeln!(formatter, "{}", Self::display_intersection(last, empty_symbols.right))?;
        }

        Ok(())
    }

    fn display_intersection(intersection: &Intersection, empty: char) -> char {
        match intersection {
            Some(Color::Black) => Self::BLACK_CIRCLE,
            Some(Color::White) => Self::WHITE_CIRCLE,
            None => empty,
        }
    }
}

#[rustfmt::skip]
impl Board {
    const BLACK_CIRCLE: char        = '\u{25EF}'; // ◯
    const WHITE_CIRCLE: char        = '\u{2B24}'; // ⬤

    const CONNECTOR: char           = '\u{2500}'; // ─

    const EMPTY_TOP_LEFT: char      = '\u{250C}'; // ┌
    const EMPTY_TOP: char           = '\u{252C}'; // ┬
    const EMPTY_TOP_RIGHT: char     = '\u{2510}'; // ┐

    const EMPTY_LEFT: char          = '\u{251C}'; // ├ 
    const EMPTY: char               = '\u{253C}'; // ┼
    const EMPTY_STAR: char          = '\u{256C}'; // ╬
    const EMPTY_RIGHT: char         = '\u{2524}'; // ┤

    const EMPTY_BOTTOM_LEFT: char   = '\u{2514}'; // └
    const EMPTY_BOTTOM: char        = '\u{2534}'; // ┴
    const EMPTY_BOTTOM_RIGHT: char  = '\u{2518}'; // ┘
    
    const EMPTY_TOP_ROW:    EmptySymbols = EmptySymbols { left: Self::EMPTY_TOP_LEFT,    mid: Self::EMPTY_TOP,    right: Self::EMPTY_TOP_RIGHT    };
    const EMPTY_ROW:        EmptySymbols = EmptySymbols { left: Self::EMPTY_LEFT,        mid: Self::EMPTY,        right: Self::EMPTY_RIGHT        };
    const EMPTY_BOTTOM_ROW: EmptySymbols = EmptySymbols { left: Self::EMPTY_BOTTOM_LEFT, mid: Self::EMPTY_BOTTOM, right: Self::EMPTY_BOTTOM_RIGHT };
 }
