use crate::common::prolog::*;

use crate::board::{Dimensions, GoNode};
use sgf_parse::go::Prop;

pub(super) struct RootNode {
    go_node: GoNode,
}

impl TryFrom<&GoNode> for RootNode {
    type Error = Error;

    fn try_from(go_node: &GoNode) -> Result<Self, Self::Error> {
        if !go_node.is_root {
            return Err(Error::message("Node is not root node"));
        }

        Ok(Self { go_node: go_node.clone() })
    }
}

impl Deref for RootNode {
    type Target = GoNode;

    fn deref(&self) -> &Self::Target {
        &self.go_node
    }
}

impl RootNode {
    pub fn get_dimensions(&self) -> Result<Dimensions, Error> {
        match self.get_property("SZ") {
            Some(&Prop::SZ((x, y))) => Ok(Dimensions { x: x.into(), y: y.into() }),
            _ => Err(Error::message("Missing SZ property")),
        }
    }

    pub fn get_players(&self) -> Result<Map<Color, String>, Error> {
        match (self.get_property("PB"), self.get_property("PW")) {
            (Some(Prop::PB(black)), Some(Prop::PW(white))) => Ok(Map::from([(Color::Black, black.to_string()), (Color::White, white.to_string())])),
            _ => Err(Error::message("Missing PB/PW property")),
        }
    }

    pub fn get_ranks(&self) -> Result<Map<Color, String>, Error> {
        match (self.get_property("BR"), self.get_property("WR")) {
            (Some(Prop::BR(black)), Some(Prop::WR(white))) => Ok(Map::from([(Color::Black, black.to_string()), (Color::White, white.to_string())])),
            _ => Err(Error::message("Missing BR/WR property")),
        }
    }

    pub fn get_komi(&self) -> Result<f64, Error> {
        match self.get_property("KM") {
            Some(&Prop::KM(komi)) => Ok(komi),
            _ => Err(Error::message("Missing KM property")),
        }
    }

    pub fn get_handicap(&self) -> Result<i64, Error> {
        match self.get_property("HA") {
            Some(&Prop::HA(handicap)) => Ok(handicap),
            _ => Err(Error::message("Missing HA property")),
        }
    }
}
