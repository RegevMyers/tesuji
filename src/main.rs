mod api;
mod board;
mod cli;
mod common;

use crate::common::prolog::*;

use clap::Parser;
use sgf_parse::go as parser;

use std::io::Read;

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut reader = io::BufReader::new(fs::File::open(path)?);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn handle_sgf(sgf_path: &Path) -> Result<(), Error> {
    log::message("Welcome to Tesuji!");

    log::input(&format!("Reading: {sgf_path:?}"));

    let sgf = read_file(sgf_path)?;
    let collection = parser::parse(&sgf)?;
    let root = collection.first().ok_or(Error::message("Empty collection"))?;
    let main_variation = root.main_variation();

    let nodes = main_variation.collect::<Vec<_>>();
    let board = board::Board::from_sgf(nodes)?;

    println!();
    println!(
        "{}[{}]: {} ({}) | {}[{}]: {}+{}",
        board.players()[&Color::Black],
        board.ranks()[&Color::Black],
        board.captures()[&Color::Black],
        board.handicap(),
        board.players()[&Color::White],
        board.ranks()[&Color::White],
        board.captures()[&Color::White],
        board.komi()
    );
    println!("{}", board);

    Ok(())
}

fn handle_id(id: u64) -> Result<(), Error> {
    let client = api::http::Client::new()?;
    let state = client.game_state(id)?;
    dbg!(state);
    todo!()
}

fn app(cli: cli::Cli) -> Result<(), Error> {
    if let Some(sgf) = cli.sgf {
        return handle_sgf(sgf.as_path());
    }

    if let Some(id) = cli.id {
        return handle_id(id);
    }

    Err(Error::message("No option given"))
}

fn main() {
    let cli = cli::Cli::parse();

    match app(cli) {
        Ok(()) => log::ok("Done"),
        Err(error) => error.log(),
    }
}
