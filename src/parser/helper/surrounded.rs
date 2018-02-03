//! A module to help with parsing blocks of potentially nested tokens, with a start and an end
//! token. For example, this would allow parsing of `{
//!     hello {
//!     }
//! }`
//!  whereas simpler methods would incorrectly stop at the first } found.

use super::super::ParseError;
use lexer::Token;
use std::slice::Iter;

/// A function to consume a list of tokens surrounded by a given starting and ending string. The
/// starting and ending tokens cannot be the same.
///
/// Will return a parse error if there is no matching end token.
///
/// The token stream should be placed on the starting token - this function will return an error if
/// not.
pub fn consume_surrounded<'a>(
    tok_stream: &mut Iter<'a, Token<'a>>,
    start: &str,
    end: &str,
) -> Result<(), ParseError> {
    debug_assert!(start != end, "Starting token cannot equal ending token");
    let tok = try!(tok_stream.next().ok_or(ParseError::new(
        "Expected token, got EOF".to_owned(),
    )));
    if tok.val != start {
        return Err(ParseError::new(
            "Expected '".to_owned() + start + "', found '" + tok.val + "'.",
        ));
    }

    let mut curr_level = 1;
    loop {
        let tok = try!(tok_stream.next().ok_or(ParseError::new(
            "Expected token, got EOF".to_owned(),
        )));
        if tok.val == start {
            curr_level += 1
        }
        else if tok.val == end {
            curr_level -= 1
        }
        if curr_level == 0 {
            return Ok(());
        }
    }
}
