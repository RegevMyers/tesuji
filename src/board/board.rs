use array2d::Array2D;
use sgf_parse::go::{Move, Point, Prop};
use sgf_parse::{PropertyType, SgfNode, SgfProp};

use crate::common::{Color, Error, SplitEnds, log};

use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub stone: Option<Color>,
    pub star: bool,
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection { stone: None, star: false }
    }
}
pub struct Board {
    board: Array2D<Intersection>,
}

#[derive(Copy, Clone)]
struct EmptySymbols {
    left: char,
    mid: char,
    right: char,
}

type GoNode = SgfNode<Prop>;

impl Board {
    pub fn new(nodes: Vec<&GoNode>) -> Result<Self, Error> {
        let root = nodes.first().ok_or(Error::message("No nodes"))?;

        let dimensions = Self::get_dimensions(root)?;
        let mut board = Self::initial_board(dimensions);

        log::info(&format!("Board | Dimensions: {:?}", dimensions));

        for node in nodes {
            let r#move = Self::get_move(node, dimensions);
            let setup_moves = Self::get_setup_moves(node);

            log::trace(&format!("Node | Move: {:?}, Setup: {:?}", r#move, setup_moves));

            if let Some((color, Some((x, y)))) = r#move {
                board[(x, y)] = Intersection { stone: Some(color), star: false };
            }

            for (color, points) in setup_moves {
                for (x, y) in points {
                    board[(x, y)] = Intersection { stone: color, star: false };
                }
            }
        }

        Ok(Self { board })
    }
}

impl Board {
    fn get_dimensions(root: &GoNode) -> Result<(usize, usize), Error> {
        if !root.is_root {
            return Err(Error::message("Node is not root node"));
        }

        match root.get_property("SZ") {
            Some(&Prop::SZ((x, y))) => Ok((x.into(), y.into())),
            _ => Err(Error::message("Missing property in root node: SZ")),
        }
    }

    fn get_stars(dimensions: (usize, usize)) -> Vec<(usize, usize)> {
        let (board_x, board_y) = dimensions;

        let rel_tl = |(x, y): (usize, usize)| (x, y);
        let rel_tr = |(x, y): (usize, usize)| (x, board_y - y - 1);
        let rel_bl = |(x, y): (usize, usize)| (board_x - x - 1, y);
        let rel_br = |(x, y): (usize, usize)| (board_x - x - 1, board_y - y - 1);

        let center = |(x, y): (usize, usize)| vec![(x / 2, y / 2)];
        let corners = |(x, y): (usize, usize)| vec![rel_tl((x, y)), rel_tr((x, y)), rel_bl((x, y)), rel_br((x, y))];
        let sides = |(x, y): (usize, usize)| vec![(x / 2, 3), (3, y / 2), (x / 2, y - 4), (x - 4, y / 2)];

        let mut stars = match dimensions {
            (x, y) if x <= 5 || y <= 5 => vec![],
            (x, y) if x <= 13 || y <= 13 => vec![corners((2, 2))],
            (x, y) => vec![corners((3, 3)), sides((x, y))],
        };

        let is_even = |(x, y)| x % 2 == 0 || y % 2 == 0;

        if !is_even(dimensions) {
            stars.push(center(dimensions))
        }

        stars.into_iter().flatten().collect()
    }

    fn get_move(node: &GoNode, dimensions: (usize, usize)) -> Option<(Color, Option<(usize, usize)>)> {
        let is_normal_board_size = {
            let (x, y) = dimensions;
            x <= 19 && y <= 19
        };

        match *node.get_move()? {
            Prop::B(Move::Move(Point { x: 19, y: 19 })) if is_normal_board_size => Some((Color::Black, None)),
            Prop::W(Move::Move(Point { x: 19, y: 19 })) if is_normal_board_size => Some((Color::White, None)),
            Prop::B(Move::Move(Point { x, y })) => Some((Color::Black, Some((x.into(), y.into())))),
            Prop::W(Move::Move(Point { x, y })) => Some((Color::White, Some((x.into(), y.into())))),
            Prop::B(Move::Pass) => Some((Color::Black, None)),
            Prop::W(Move::Pass) => Some((Color::White, None)),
            _ => None,
        }
    }

    fn get_setup_moves(node: &GoNode) -> Vec<(Option<Color>, Vec<(usize, usize)>)> {
        let setup_properties = node.properties().find(|prop| prop.property_type() == Some(PropertyType::Setup));

        let to_coordinates = |points: &HashSet<Point>| points.into_iter().map(|&Point { x, y }| (x.into(), y.into())).collect();

        setup_properties
            .into_iter()
            .filter_map(|prop| match prop {
                Prop::AB(points) => Some((Some(Color::Black), to_coordinates(points))),
                Prop::AW(points) => Some((Some(Color::White), to_coordinates(points))),
                Prop::AE(points) => Some((None, to_coordinates(points))),
                _ => None,
            })
            .collect()
    }
}

impl Board {
    fn initial_board(dimensions: (usize, usize)) -> Array2D<Intersection> {
        let (x, y) = dimensions;
        let mut board = Array2D::filled_with(Intersection::default(), x, y);

        for star in Self::get_stars(dimensions) {
            board[star].star = true;
        }

        board
    }
}

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
            Some(Color::Black) => Self::BLACK_CIRCLE,
            Some(Color::White) => Self::WHITE_CIRCLE,
            None if intersection.star => Self::STAR,
            None => empty,
        };

        write!(formatter, "{}{}", symbol, connector)?;

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
