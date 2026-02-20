use crate::sgf::parser::parsers::value;

use crate::sgf::parser;

#[derive(Debug)]
struct Property {
    identifier: String,
    values: Vec<parser::Value>,
}

// pub fn property(string: &str) -> Result<u8, parser::Error> { }

//fn property_identifier

//fn property_value(string: &str, parser: parser::Parser) -> Result<parser::Value, parser::Error> {
//    if !string.starts_with("[") || !string.ends_with("]") {
//        return Err(parser::Error::new("Property Value", string, "property values must be surrounded with square brackets (\"[]\")"));
//    }

//    parser(string)
//}
