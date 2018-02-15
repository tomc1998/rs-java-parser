//! Helper module to parse modifiers, i.e. public / static / private etc

use lexer::Token;
use std::slice::Iter;
use super::super::ParseError;
use java_model::*;

/// Returns Some if modifier detected - None if no modifier or EOF
pub fn try_parse_modifier<'a>(tok_stream: &mut Iter<'a, Token<'a>>) -> Option<Modifier> {
    let tok = tok_stream.as_slice().first();
    if tok.is_none() {
        return None;
    }
    let tok = tok.unwrap();
    let modifier = match tok.val {
        "transient" => Some(Modifier::Transient),
        "volatile" => Some(Modifier::Volatile),
        "public" => Some(Modifier::Public),
        "protected" => Some(Modifier::Protected),
        "private" => Some(Modifier::Private),
        "abstract" => Some(Modifier::Abstract),
        "static" => Some(Modifier::Static),
        "final" => Some(Modifier::Final),
        "synchronized" => Some(Modifier::Synchronized),
        "native" => Some(Modifier::Native),
        "strictfp" => Some(Modifier::Strictfp),
        _ => None,
    };
    if modifier.is_some() {
        tok_stream.next().unwrap();
    }
    return modifier;
}

pub fn parse_modifiers<'a>(
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Result<Vec<Modifier>, ParseError> {
    let mut modifiers = Vec::new();

    loop {
        let modifier = try_parse_modifier(tok_stream);
        if modifier.is_some() {
            modifiers.push(modifier.unwrap());
        } else {
            break;
        }
    }

    return Ok(modifiers);
}
