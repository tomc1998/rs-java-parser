#![allow(dead_code)]

use super::*;
use lexer::{Token, TokenType};

/// Parses if the given token matches the val and type given - if NOT, consumes
/// nothing and returns None.
pub fn consume_maybe(tokens: &mut TokenIter, src: &str,
                   exp_val: &str, exp_type: TokenType) -> Option<Token> {
    if let Some(tok) = tokens.clone().next() {
        if tok.token_type == exp_type && tok.val(src) == exp_val {
            Some(*tokens.next().unwrap())
        } else {
            None
        }
    } else {
        None
    }
}
