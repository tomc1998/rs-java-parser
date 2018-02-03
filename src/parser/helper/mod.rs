//! A module containing loads of helper functions for parsing.

mod modifiers;
mod surrounded;

pub use self::modifiers::{Modifier, parse_modifiers, try_parse_modifier};
pub use self::surrounded::consume_surrounded;

use lexer::{TokenType, Token};
use std::slice::Iter;

/// Parse identifier tokens in a comma-separated list until a token breaks the pattern. Will not
/// consume the token that breaks the pattern.
///
/// List cannot start on the comma - nothing will be returned in this case.
pub fn parse_comma_separated_identifier_list<'a>(
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Vec<&'a str> {
    let mut identifiers = Vec::new();
    loop {
        {
            let tok = tok_stream.as_slice().first();
            if tok.is_none() {
                return identifiers;
            }
            let tok = tok.unwrap();
            if tok.token_type != TokenType::Ident {
                return identifiers;
            }
        }
        identifiers.push(tok_stream.next().unwrap().val);

        {
            let tok = tok_stream.as_slice().first();
            if tok.is_none() {
                return identifiers;
            }
            let tok = tok.unwrap();
            if tok.token_type != TokenType::Punc || tok.val != "," {
                return identifiers;
            }
        }
        tok_stream.next();
    }

}
