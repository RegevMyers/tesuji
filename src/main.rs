mod board;
mod cli;
mod common;

use crate::board::Board;

use clap::Parser;
use sgf_parse::go as parser;

use common::{Error, log};

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

fn app(sgf_path: &Path) -> Result<(), Error> {
    log::message("Welcome to Tesuji!");

    log::input(&format!("Reading: {:?}", sgf_path));

    let sgf = read_file(sgf_path)?;
    let collection = parser::parse(&sgf)?;
    let root = collection.first().ok_or(Error::message("Empty collection"))?;
    let main_variation = root.main_variation();

    let board = Board::new(main_variation.collect())?;
    println!("\n{}", board);

    Ok(())
}

fn main() {
    let cli = cli::Cli::parse();

    match app(cli.sgf.as_path()) {
        Ok(()) => log::ok("Done"),
        Err(error) => error.log(),
    }
}
