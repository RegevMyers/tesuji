mod sgf;
mod common;

use sgf_parse::go as parser;

use common::log;

use std::path::Path;
use std::fs;
use std::io;
use std::io::{Read};
use std::env;

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut reader = io::BufReader::new(fs::File::open(&path)?);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn main() -> Result<(), common::Error> {
    log::location("Main");

    let args: Vec<String> = env::args().collect();
    log::input(&format!("Cmd: {:?}", args));

    log::location(&format!("Reading: {}", args[1]));

    let sgf = read_file(Path::new(&args[1]))?;
    let collection = parser::parse(&sgf)?;
    let root = collection.first().ok_or(common::Error::new("Empty collection"))?;
    let main_variation = root.main_variation();

    sgf::Board::new(main_variation.collect());

//  for node in main_variation {
//      if let Some(r#move) = node.get_move() {
//          match r#move {
//              sgf::Prop::B(sgf::Move::Pass) => log::output("(B) Pass"),
//              sgf::Prop::W(sgf::Move::Pass) => log::output("(W) Pass"),
//              sgf::Prop::B(sgf::Move::Move(sgf::Point{ x, y })) => log::output(&format!("(B) {}-{}", x, y)),
//              sgf::Prop::W(sgf::Move::Move(sgf::Point{ x, y })) => log::output(&format!("(W) {}-{}", x, y)),
//              _ => todo!() 
//          }
//      }
//  }
///    let (a, rest): (Value, String) = compose(number, number, "6:3a")?;
//    log::ok(&format!("6:3 -> {:?}", a));
//  
    log::ok("Done");
    Ok(())
}
