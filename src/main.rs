mod board;
mod cli;
mod common;

use clap::Parser;
use sgf_parse::go as parser;

use common::{Color, Error, log};

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

    log::input(&format!("Reading: {sgf_path:?}"));

    let sgf = read_file(sgf_path)?;
    let collection = parser::parse(&sgf)?;
    let root = collection.first().ok_or(Error::message("Empty collection"))?;
    let main_variation = root.main_variation();

    let nodes = main_variation.collect::<Vec<_>>();
    let (root, rest) = nodes.split_first().ok_or(Error::message("No root node"))?;

    let mut board = board::Board::new(root)?;
    board.apply_nodes(rest.to_vec())?;

    println!();
    println!("Black: {} | White: {}", board.captures()[&Color::Black], board.captures()[&Color::White]);
    println!("{}", board);

    Ok(())
}

fn main() {
    let cli = cli::Cli::parse();

    match app(cli.sgf.as_path()) {
        Ok(()) => log::ok("Done"),
        Err(error) => error.log(),
    }
}
