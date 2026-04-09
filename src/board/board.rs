use array2d::Array2D;
use sgf_parse::go::{Move, Point as SgfPoint, Prop};
use sgf_parse::{PropertyType, SgfNode, SgfProp};

use crate::common::{Color, Error, SplitEnds, log};

use std::collections::{HashMap as Map, HashSet as Set};
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

#[derive(Debug)]
struct Dimensions {
    x: usize,
    y: usize,
}

pub struct Board {
    board: Array2D<Intersection>,
    dimensions: Dimensions,
    captures: Map<Color, usize>,
}

type GoNode = SgfNode<Prop>;

type Point = (usize, usize);
type Group = Set<Point>;

impl Board {
    pub fn new(root: &GoNode) -> Result<Self, Error> {
        let dimensions = Self::get_dimensions(root)?;
        let board = Self::initial_board(&dimensions);

        Ok(Self { board, dimensions, captures: Map::from([(Color::Black, 0), (Color::White, 0)]) })
    }

    pub fn apply_nodes(&mut self, nodes: Vec<&GoNode>) -> Result<(), Error> {
        for node in nodes {
            let dimensions = &self.dimensions;

            let r#move = Self::get_move(node, dimensions);
            let setup_moves = Self::get_setup_moves(node);

            log::trace(&format!("Node | Move: {move:?}, Setup: {setup_moves:?}"));

            if let Some((color, Some((x, y)))) = r#move {
                self.play_move(color, (x, y))?;
            }

            for (color, points) in setup_moves {
                for (x, y) in points {
                    self.board[(x, y)] = Intersection { stone: color, star: false };
                }
            }
        }

        Ok(())
    }
}

impl Board {
    fn initial_board(dimensions: &Dimensions) -> Array2D<Intersection> {
        let &Dimensions { x, y } = dimensions;
        let mut board = Array2D::filled_with(Intersection::default(), x, y);

        for star in Self::get_stars(dimensions) {
            board[star].star = true;
        }

        board
    }
}

type SetupMove = (Option<Color>, Vec<Point>);

impl Board {
    fn get_dimensions(root: &GoNode) -> Result<Dimensions, Error> {
        if !root.is_root {
            return Err(Error::message("Node is not root node"));
        }

        match root.get_property("SZ") {
            Some(&Prop::SZ((x, y))) => Ok(Dimensions { x: x.into(), y: y.into() }),
            _ => Err(Error::message("Missing property in root node: SZ")),
        }
    }

    fn get_stars(dimensions: &Dimensions) -> Vec<Point> {
        let &Dimensions { x: board_x, y: board_y } = dimensions;

        let top_left = |(x, y): Point| (x, y);
        let top_right = |(x, y): Point| (x, board_y - y - 1);
        let bottom_left = |(x, y): Point| (board_x - x - 1, y);
        let bottom_right = |(x, y): Point| (board_x - x - 1, board_y - y - 1);

        let center = |(x, y): Point| vec![(x / 2, y / 2)];
        let corners = |(x, y): Point| vec![top_left((x, y)), top_right((x, y)), bottom_left((x, y)), bottom_right((x, y))];
        let sides = |(x, y): Point| vec![(x / 2, 3), (3, y / 2), (x / 2, y - 4), (x - 4, y / 2)];

        let mut stars = match dimensions {
            &Dimensions { x, y } if x <= 5 || y <= 5 => vec![],
            &Dimensions { x, y } if x <= 13 || y <= 13 => vec![corners((2, 2))],
            &Dimensions { x, y } => vec![corners((3, 3)), sides((x, y))],
        };

        let is_even = |&Dimensions { x, y }| x % 2 == 0 || y % 2 == 0;

        if !is_even(dimensions) {
            let &Dimensions { x, y } = dimensions;
            stars.push(center((x, y)))
        }

        stars.into_iter().flatten().collect()
    }

    fn get_move(node: &GoNode, dimensions: &Dimensions) -> Option<(Color, Option<Point>)> {
        let is_normal_board_size = {
            let &Dimensions { x, y } = dimensions;
            x <= 19 && y <= 19
        };

        match node.get_move()? {
            &Prop::B(Move::Pass) => Some((Color::Black, None)),
            &Prop::W(Move::Pass) => Some((Color::White, None)),
            &Prop::B(Move::Move(SgfPoint { x: 19, y: 19 })) if is_normal_board_size => Some((Color::Black, None)),
            &Prop::W(Move::Move(SgfPoint { x: 19, y: 19 })) if is_normal_board_size => Some((Color::White, None)),
            &Prop::B(Move::Move(SgfPoint { x, y })) => Some((Color::Black, Some((x.into(), y.into())))),
            &Prop::W(Move::Move(SgfPoint { x, y })) => Some((Color::White, Some((x.into(), y.into())))),
            _ => None,
        }
    }

    fn get_setup_moves(node: &GoNode) -> Vec<SetupMove> {
        let setup_properties = node.properties().find(|prop| prop.property_type() == Some(PropertyType::Setup));

        let to_points = |points: &Set<SgfPoint>| points.iter().map(|&SgfPoint { x, y }| (x.into(), y.into())).collect();

        setup_properties
            .into_iter()
            .filter_map(|prop| match prop {
                Prop::AB(points) => Some((Some(Color::Black), to_points(points))),
                Prop::AW(points) => Some((Some(Color::White), to_points(points))),
                Prop::AE(points) => Some((None, to_points(points))),
                _ => None,
            })
            .collect()
    }
}

impl Board {
    fn play_move(&mut self, color: Color, point: Point) -> Result<(), Error> {
        self.board[point].stone = Some(color);

        let adjacent_points = self.get_adjacent_points(point);
        let adjacent_groups = adjacent_points.into_iter().map(|point| self.get_containing_group(point)).flatten().collect::<Vec<Group>>();
        log::trace(&format!("Play | Adjacent groups: {adjacent_groups:?}"));

        for group in adjacent_groups {
            self.try_capture(group)
        }

        if let Some(own_group) = self.get_containing_group(point) {
            self.try_capture(own_group);
        }

        Ok(())
    }

    fn get_adjacent_points(&self, point: Point) -> Set<Point> {
        let (x, y) = point;
        let Dimensions { x: board_x, y: board_y } = self.dimensions;

        Set::from_iter(
            [
                (x != 0).then_some((x - 1, y)),
                (y != 0).then_some((x, y - 1)),
                (x != board_x - 1).then_some((x + 1, y)),
                (y != board_y - 1).then_some((x, y + 1)),
            ]
            .into_iter()
            .flatten(),
        )
    }

    fn get_containing_group(&self, point: Point) -> Option<Group> {
        let mut group = Group::new();

        self.add_and_recurse(&mut group, point);

        if group.is_empty() {
            return None;
        }

        Some(group)
    }

    fn add_and_recurse(&self, group: &mut Group, point: Point) {
        let point_color = self.board[point].stone;

        if group.contains(&point) || point_color == None {
            return;
        }

        group.insert(point);

        for adjacent_point in self.get_adjacent_points(point) {
            let adjacent_point_color = self.board[adjacent_point].stone;
            if adjacent_point_color == point_color {
                self.add_and_recurse(group, adjacent_point);
            }
        }
    }

    fn try_capture(&mut self, group: Group) {
        if self.is_alive(&group) {
            return;
        };

        for point in group {
            if let Some(color) = self.board[point].stone {
                self.captures.insert(color, self.captures[&color] + 1);
                self.board[point].stone = None;
            }
        }
    }

    fn is_alive(&self, group: &Group) -> bool {
        let group_and_bordering = group.iter().map(|point: &Point| self.get_adjacent_points(*point)).flatten().collect::<Group>();
        let bordering = &group_and_bordering - group;

        bordering.into_iter().find(|point: &Point| self.board[*point].stone == None).is_some()
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
