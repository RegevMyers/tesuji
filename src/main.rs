mod sgf;
mod common;

use sgf::parser::parsers::collection::Collection;
use common::logging as log;

use std::path::Path;

use std::fs;
use std::io;
use std::io::{Read};
use std::env;

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let mut reader = io::BufReader::new(fs::File::open(&path)?);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn main() -> Result<(), common::error::Error> {
    log::location("Main");
    log::input(&format!("Args: {:?}", env::args().collect::<Vec<_>>()));

    log::location(&format!("Reading: {}", env::args().next().unwrap()));
    let c: Collection = "wajawaja".parse()?;
    // let content = read_file(Path::new("ff4_ex.sgf"))?;
    dbg!(c);
    
    log::ok("Done");
    Ok(())
}

