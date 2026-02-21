mod sgf;
mod common;

use sgf::parser::*;
use common::logging as log;

use nom::Finish;

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

fn main() -> Result<(), common::Error> {
    log::location("Main");

    let args: Vec<String> = env::args().collect();
    log::input(&format!("Cmd: {:?}", args));

    log::location(&format!("Reading: {}", args[1]));
    let (rest, a): (&str, Value) = compose(number, number, "6:3a").finish().unwrap();
    log::ok(&format!("6:3 -> {:?}", a));
    
    log::ok("Done");
    Ok(())
}

