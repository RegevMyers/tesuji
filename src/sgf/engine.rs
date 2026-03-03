use array2d::Array2D;
use sgf_parse::SgfNode;
use sgf_parse::go::{ parse, Prop };

use crate::common::{ Color, Error };
use crate::common::log;

pub struct Intersection {
    stone: Option<Color>
}

pub struct Board {
    board: Array2D<Intersection>
}

type GoNode = SgfNode<Prop>;

impl Board {
    pub fn new(nodes: Vec<&GoNode>) -> Result<Self, Error> {
        let root = nodes.first().ok_or(Error::new("No nodes"))?;
        log::info(&format!("Root Node Data: {:?}", root.properties().collect::<Vec<&Prop>>()));
        
        if let Prop::SZ((x, y)) = root.get_property("SZ").ok_or(Error::new("Missing property: SZ"))? {
            log::info(&format!("Board Data | Size: {}-{}", x, y));
        }

        Err(Error::new("waka wak"))
    }
}

