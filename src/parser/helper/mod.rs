//! A module containing loads of helper functions for parsing.

use lexer::{TokenType, Token};

/// Parse identifier tokens in a comma-separated list until a token breaks the pattern. Will not
/// consume the token that breaks the pattern.
///
/// List cannot start on the comma - nothing will be returned in this case.
pub fn parse_comma_separated_identifier_list<'a, I>(tok_stream: &mut I) -> Vec<&'a str>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut tok_stream = tok_stream.peekable();
    let mut identifiers = Vec::new();
    loop {
        {
            let tok = tok_stream.peek();
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
            let tok = tok_stream.peek();
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
