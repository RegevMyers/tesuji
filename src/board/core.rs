use crate::common::prolog::*;

use crate::board::root_node::RootNode;

use array2d::Array2D;
use sgf_parse::go::{Move, Point as SgfPoint, Prop};
use sgf_parse::{PropertyType, SgfNode, SgfProp};

#[derive(Copy, Clone, Default)]
pub(super) struct Intersection {
    pub(super) stone: Option<Color>,
    pub(super) star: bool,
}

#[derive(Debug)]
pub(super) struct Dimensions {
    pub(super) x: usize,
    pub(super) y: usize,
}

pub struct Board {
    pub(super) board: Array2D<Intersection>,
    pub(super) dimensions: Dimensions,
    pub(super) captures: Map<Color, usize>,
    pub(super) players: Map<Color, String>,
    pub(super) ranks: Map<Color, String>,
    pub(super) komi: f64,
    pub(super) handicap: i64,
}

pub(super) type GoNode = SgfNode<Prop>;

pub(super) type Point = (usize, usize);
pub(super) type Group = Set<Point>;

impl Board {
    pub fn new(root: &GoNode) -> Result<Self, Error> {
        let root = RootNode::try_from(root)?;

        let dimensions = root.get_dimensions()?;
        let players = root.get_players()?;
        let ranks = root.get_ranks()?;
        let komi = root.get_komi()?;
        let handicap = root.get_handicap()?;

        let board = Self::initial_board(&dimensions);
        let captures = Map::from([(Color::Black, 0), (Color::White, 0)]);

        Ok(Self { board, dimensions, captures, players, ranks, komi, handicap })
    }

    pub fn apply_nodes(&mut self, nodes: Vec<&GoNode>) -> Result<(), Error> {
        for node in nodes {
            let dimensions = &self.dimensions;

            let r#move = Self::get_move(node, dimensions);
            let setup_moves = Self::get_setup_moves(node);

            if let Some((color, Some((x, y)))) = r#move {
                self.play_move(color, (x, y))?;
            }

            for (color, points) in setup_moves {
                for (x, y) in points {
                    log::trace(&format!("Setup | {color:?} @ [({x}, {y})]"));
                    self.board[(x, y)] = Intersection { stone: color, star: false };
                }
            }
        }

        Ok(())
    }

    pub fn captures(&self) -> &Map<Color, usize> {
        &self.captures
    }

    pub fn players(&self) -> &Map<Color, String> {
        &self.players
    }

    pub fn ranks(&self) -> &Map<Color, String> {
        &self.ranks
    }

    pub fn komi(&self) -> f64 {
        self.komi
    }

    pub fn handicap(&self) -> i64 {
        self.handicap
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
    fn get_stars(dimensions: &Dimensions) -> Vec<Point> {
        let &Dimensions { x: board_x, y: board_y } = dimensions;

        let top_left = |(x, y): Point| (x, y);
        let top_right = |(x, y): Point| (x, board_y - y - 1);
        let bottom_left = |(x, y): Point| (board_x - x - 1, y);
        let bottom_right = |(x, y): Point| (board_x - x - 1, board_y - y - 1);

        let center = |(x, y): (usize, usize)| vec![(x / 2, y / 2)];
        let corners = |(x, y): (usize, usize)| vec![top_left((x, y)), top_right((x, y)), bottom_left((x, y)), bottom_right((x, y))];
        let sides = |(x, y): (usize, usize)| vec![(x / 2, 3), (3, y / 2), (x / 2, y - 4), (x - 4, y / 2)];

        let mut stars = match *dimensions {
            Dimensions { x, y } if x <= 5 || y <= 5 => vec![],
            Dimensions { x, y } if x <= 13 || y <= 13 => vec![corners((2, 2))],
            Dimensions { x, y } => vec![corners((3, 3)), sides((x, y))],
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

        match *node.get_move()? {
            Prop::B(Move::Pass) => Some((Color::Black, None)),
            Prop::W(Move::Pass) => Some((Color::White, None)),
            Prop::B(Move::Move(SgfPoint { x: 19, y: 19 })) if is_normal_board_size => Some((Color::Black, None)),
            Prop::W(Move::Move(SgfPoint { x: 19, y: 19 })) if is_normal_board_size => Some((Color::White, None)),
            Prop::B(Move::Move(SgfPoint { x, y })) => Some((Color::Black, Some((x.into(), y.into())))),
            Prop::W(Move::Move(SgfPoint { x, y })) => Some((Color::White, Some((x.into(), y.into())))),
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
