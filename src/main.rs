mod cli;
mod common;
mod sgf;

use clap::Parser;
use sgf_parse::go as parser;

use common::log;

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

fn app() -> Result<(), common::Error> {
    let cli = cli::Cli::parse();
    log::location(&format!("Reading: {}", &cli.sgf));

    let sgf = read_file(Path::new(&cli.sgf))?;
    let collection = parser::parse(&sgf)?;
    let root = collection.first().ok_or(common::Error::new("Empty collection"))?;
    let main_variation = root.main_variation();

    match sgf::Board::new(main_variation.collect()) {
        Ok(board) => println!("\n{}", board),
        Err(error) => error.log(),
    }

    Ok(())
}

fn main() {
    log::location("Main");

    match app() {
        Ok(()) => log::ok("Done"),
        Err(error) => error.log(),
    }
}
