use array2d::Array2D;
use sgf_parse::{ SgfNode, SgfProp, PropertyType };
use sgf_parse::go::{ Prop, Move, Point };

use crate::common::{ log, Error, Color };

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    stone: Option<Color>,
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

impl Board {
    pub fn new(nodes: Vec<&GoNode>) -> Result<Self, Error> {
        let root = nodes.first().ok_or(Error::new("No nodes"))?;

        let (board_x, board_y) = Self::get_board_dimensions(root)?;
        let mut board = Array2D::filled_with(Intersection{ stone: None }, board_x, board_y);

        log::info(&format!("Board | Dimensions: {}-{}", board_x, board_y));

        for node in nodes {
            log::trace(&format!("Node | Move: {:?}, Setup: {:?}", Self::get_move(node, (board_x, board_y)), Self::get_setups(node)));

            if let Some((color, Move::Move(Point{ x, y }))) = Self::get_move(node, (board_x, board_y)) {
                board[(x, y)] = Intersection{ stone: Some(color) };
            }

            let setups = Self::get_setups(node);

            for (color, points) in setups {
                for Point{ x, y } in points {
                    board[(x, y)] = Intersection{ stone: color };
                }
            }
        }

        Ok(Self{ board })
    }
}

type GoNode = SgfNode<Prop>;

impl Board {
    fn get_board_dimensions(root: &GoNode) -> Result<(usize, usize), Error> {
        if !root.is_root {
            return Err(Error::new("Node is not root node"))
        }

        match root.get_property("SZ") {
            Some(&Prop::SZ((x, y))) => Ok((x, y)),
            _ => Err(Error::new("Missing property in root node: SZ")),
        }
    }

    fn get_move(node: &GoNode, board_dimensions: (usize, usize)) -> Option<(Color, Move)> {
        let is_normal_board_size = {
            let (x, y) = board_dimensions;
            x <= 19 && y <= 19
        };

        match node.get_move()? {
            &Prop::B(Move::Move(Point{ x: 19, y: 19 })) if is_normal_board_size => Some((Color::Black, Move::Pass)),
            &Prop::W(Move::Move(Point{ x: 19, y: 19 })) if is_normal_board_size => Some((Color::White, Move::Pass)),
            &Prop::B(r#move) => Some((Color::Black, r#move)),
            &Prop::W(r#move) => Some((Color::White, r#move)),
            _ => None
        }
    }

    fn get_setups(node: &GoNode) -> Vec<(Option<Color>, HashSet<Point>)>{
        let setup_properties = node.properties().find(|prop| prop.property_type() == Some(PropertyType::Setup));

        setup_properties.into_iter().filter_map(|prop| {
            match prop {
                Prop::AB(points) => Some((Some(Color::Black), points.clone())),
                Prop::AW(points) => Some((Some(Color::White), points.clone())),
                Prop::AE(points) => Some((None, points.clone())),
                _ => None
            }
        }).collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let empty_top    = EmptySymbols{ left: Self::EMPTY_TOP_LEFT,    mid: Self::EMPTY_TOP,    right: Self::EMPTY_TOP_RIGHT    };
        let empty_mid    = EmptySymbols{ left: Self::EMPTY_LEFT,        mid: Self::EMPTY,        right: Self::EMPTY_RIGHT        };
        let empty_bottom = EmptySymbols{ left: Self::EMPTY_BOTTOM_LEFT, mid: Self::EMPTY_BOTTOM, right: Self::EMPTY_BOTTOM_RIGHT };

        let mut rows = self.board.columns_iter().peekable();

        if let Some(row) = rows.next() {
            Self::print_column(row.collect(), empty_top, formatter)?;
        }

        while let Some(row) = rows.next() {
            if !rows.peek().is_none() {
                Self::print_column(row.collect(), empty_mid, formatter)?;
            }
            else {
                Self::print_column(row.collect(), empty_bottom, formatter)?;
            }
        }

        Ok(())
    }
}

impl Board {
    fn print_column(row: Vec<&Intersection>, empty_symbols: EmptySymbols, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut intersections = row.into_iter().peekable();

        if let Some(intersection) = intersections.next() {
            write!(formatter, "{}{}", Self::display_intersection(intersection, empty_symbols.left), Self::CONNECTOR)?;
        }

        while let Some(intersection) = intersections.next() {
            if !intersections.peek().is_none() {
                write!(formatter, "{}{}", Self::display_intersection(intersection, empty_symbols.mid), Self::CONNECTOR)?;
            }
            else {
                write!(formatter, "{}\n", Self::display_intersection(intersection, empty_symbols.right))?;
            }
        }

        Ok(())
    }

    fn display_intersection(intersection: &Intersection, empty: char) -> char {
        match intersection.stone {
            Some(Color::Black) => Self::BLACK_CIRCLE,
            Some(Color::White) => Self::WHITE_CIRCLE,
            None               => empty,
        }
    }
}

impl Board {
    const BLACK_CIRCLE: char        = '\u{25EF}'; // ◯
    const WHITE_CIRCLE: char        = '\u{2B24}'; // ⬤

    const CONNECTOR: char           = '\u{2500}'; // ─

    const EMPTY_TOP_LEFT: char      = '\u{250C}'; // ┌
    const EMPTY_TOP: char           = '\u{252C}'; // ┬
    const EMPTY_TOP_RIGHT: char     = '\u{2510}'; // ┐

    const EMPTY_LEFT: char          = '\u{251C}'; // ├ 
    const EMPTY: char               = '\u{253C}'; // ┼
    const EMPTY_RIGHT: char         = '\u{2524}'; // ┤

    const EMPTY_BOTTOM_LEFT: char   = '\u{2514}'; // └
    const EMPTY_BOTTOM: char        = '\u{2534}'; // ┴
    const EMPTY_BOTTOM_RIGHT: char  = '\u{2518}'; // ┘
}

