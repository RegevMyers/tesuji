use array2d::Array2D;
use sgf_parse::{ SgfNode, SgfProp, PropertyType };
use sgf_parse::go::{ Prop, Move, Point };

use crate::common::{ log, Error, Color };

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Intersection {
    stone: Option<Color>
}

pub struct Board {
    board: Array2D<Intersection>
}

impl Board {
    const: WHITE_CIRCLE         = '\u{25EF}'; // ◯
    const: BLACK_CIRCLE         = '\u{2B24}'; // ⬤
    
    const: CONNECTOR            = '\u{2500}'; // ─

    const: EMPTY                = '\u{253C}'; // ┼
    const: EMPTY_LEFT           = '\u{251C}'; // ├ 
    const: EMPTY_RIGHT          = '\u{2524}'; // ┤
    const: EMPTY_TOP            = '\u{252C}'; // ┬
    const: EMPTY_BOTTOM         = '\u{2534}'; // ┴
    const: EMPTY_TOP_LEFT       = '\u{250C}'; // ┌
    const: EMPTY_TOP_RIGHT      = '\u{2510}'; // ┐
    const: EMPTY_BOTTOM_LEFT    = '\u{2514}'; // └
    const: EMPTY_BOTTOM_LEFT    = '\u{2518}'; // ┘
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = self.board.rows_iter().peekable();

        print_first_row(rows.next()?, formatter); 

        while let Some(row) = rows.next() {
            if row.peek().is_none() {
                print_last_row();
            }
            else {
                print_row();
            }

            write!(formatter, "\n");
        }

        Ok(())
    }

    fn print_first_row(row: Vec<Intersection>) -> fmt::Result {
        for intersection in row {

        }
    }
    
    struct EmptySymbols {
        first: char,
        mid: char,
        last: char,
    }

    fn print_row(row: Vec<Intersection>, empty_symbols: EmptySymbols) {
        let mut intersections = row.into_iter().peekable();

        write!(formatter, "{}{}", display_intersection(intersections.next()?), Self::CONNECTOR);
    }

    fn display_intersection(intersection: Intersection, empty: char) -> char {
        match intersection {
            Some(Color::Black) => Self::BLACK_CIRCLE,
            Some(Color::White) => Self::WHITE_CIRCLE,
            None               => empty,
        }
    }
}

type GoNode = SgfNode<Prop>;

impl Board {
    pub fn new(nodes: Vec<&GoNode>) -> Result<Self, Error> {
        let root = nodes.first().ok_or(Error::new("No nodes"))?;

        let (board_x, board_y) = get_board_dimensions(root)?;

        log::info(&format!("Board Data | Dimensions: {}-{}", board_x, board_y));

        let mut board = Array2D::filled_with(Intersection{ stone: None }, board_x.into(), board_y.into());

        for node in nodes {
            log::trace(&format!("Node | Move: {:?}, Setup: {:?}", get_move(node, (board_x, board_y)), get_setups(node)));

            if let Some((color, Move::Move(Point{ x, y }))) = get_move(node, (board_x, board_y)) {
                board[(x.into(), y.into())] = Intersection{ stone: Some(color) };
            }
        }

        Ok(Self{ board })
    }
}

fn get_board_dimensions(root: &GoNode) -> Result<(u8, u8), Error> {
    if !root.is_root {
        return Err(Error::new("Node is not root node"))
    }

    match root.get_property("SZ") {
        Some(&Prop::SZ((x, y))) => Ok((x, y)),
        _ => Err(Error::new("Missing property in root node: SZ")),
    }
}

fn get_move(node: &GoNode, board_dimensions: (u8, u8)) -> Option<(Color, Move)> {
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
