//! This module, unlike the lexer module, does NOT contain the java parser. It instead contains
//! many parsers which can be used to extract different data from a token stream.

pub mod declarations;
pub mod class;
pub mod helper;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: String) -> ParseError {
        ParseError {
            message: message,
        }
    }
}
