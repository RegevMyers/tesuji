use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

mod sgf;
mod common;

use sgf::parser::parsers::collection::Collection;

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let mut reader = BufReader::new(File::open(&path)?);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn main() -> Result<(), common::error::Error> {
    let c: Collection = "wajawaja".parse()?;
    // let content = read_file(Path::new("ff4_ex.sgf"))?;
    dbg!(c);
    println!("[ + ] Done");

    Ok(())
}
