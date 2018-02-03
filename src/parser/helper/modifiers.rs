//! Helper module to parse modifiers, i.e. public / static / private etc

use lexer::Token;
use std::slice::Iter;
use super::super::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
    Transient,
    Volatile,
    Public,
    Protected,
    Private,
    Abstract,
    Static,
    Final,
    Synchronized,
    Native,
    Strictfp,
}

pub fn parse_modifiers<'a>(
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Result<Vec<Modifier>, ParseError> {
    let mut modifiers = Vec::new();

    loop {
        let tok = try!(tok_stream.as_slice().first().ok_or(ParseError::new(
            "Expected modifier, found EOF".to_owned(),
        )));
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
            modifiers.push(modifier.unwrap());
            tok_stream.next().unwrap();
        } else {
            break;
        }
    }

    return Ok(modifiers);
}
