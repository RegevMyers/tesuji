mod common;
mod sgf;

use sgf_parse::go as parser;

use common::log;

use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut reader = io::BufReader::new(fs::File::open(path)?);
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

    match sgf::Board::new(main_variation.collect()) {
        Ok(board) => println!("\n{}", board),
        Err(error) => error.log(),
    }

    log::ok("Done");

    Ok(())
}
