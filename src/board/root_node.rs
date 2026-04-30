use crate::common::prolog::*;

use crate::board::GoNode;
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
    pub fn get_players(&self) -> Result<Map<Color, String>, Error> {
        if let Some(Prop::PB(black)) = self.get_property("PB")
            && let Some(Prop::PW(white)) = self.get_property("PW")
        {
            return Ok(Map::from([(Color::Black, black.to_string()), (Color::White, white.to_string())]));
        }

        Err(Error::message("Missing PB/PW property"))
    }

    pub fn get_ranks(&self) -> Result<Map<Color, String>, Error> {
        if let Some(Prop::BR(black)) = self.get_property("BR")
            && let Some(Prop::WR(white)) = self.get_property("WR")
        {
            return Ok(Map::from([(Color::Black, black.to_string()), (Color::White, white.to_string())]));
        }

        Err(Error::message("Missing BR/WR property"))
    }

    pub fn get_komi(&self) -> Result<f64, Error> {
        if let Some(&Prop::KM(komi)) = self.get_property("KM") {
            return Ok(komi);
        }

        Err(Error::message("Missing KM property"))
    }

    pub fn get_handicap(&self) -> Result<i64, Error> {
        if let Some(&Prop::HA(handicap)) = self.get_property("HA") {
            return Ok(handicap);
        }

        Err(Error::message("Missing HA property"))
    }
}
