mod token;
mod keywords;

pub use self::token::{Token, TokenType};

use std;

pub type CharStream<'a> = std::str::Chars<'a>;

/// Lex a given source file, and return a list of tokens.
pub fn lex_str<'a>(input: &'a str) -> Vec<Token<'a>> {
    lex_char_stream(input.chars())
}

/// Lex a given source file, and return a list of tokens.
pub fn lex_char_stream<'a>(mut input: CharStream<'a>) -> Vec<Token<'a>> {
    let mut token_list = Vec::new();

    while input.as_str().len() > 0 {
        let mut _token;

        // Package declarations
        _token = keywords::lex(&mut input);
        if _token.is_some() {
            token_list.push(_token.unwrap());
            continue;
        }
    }

    return token_list;
}
