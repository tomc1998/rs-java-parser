//! Helper module to parse modifiers, i.e. public / static / private etc

use lexer::Token;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modifier;

pub fn parse_modifiers<'a>(tok_stream: &mut Iter<'a, Token<'a>>) -> Vec<Modifier> {
    let modifiers = Vec::new();

    return modifiers;
}
