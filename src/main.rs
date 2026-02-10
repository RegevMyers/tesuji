use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

mod sgf;

use sgf::parser::parsers::collection::Collection;

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let mut reader = BufReader::new(File::open(&path)?);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn main() -> Result<(), std::io::Error> {
    let mut c: Collection;
    let content = read_file(Path::new("ff4_ex.sgf"))?;
    println!("{}", content);

    Ok(())
}
