//! Module for parsing class declarations

use super::ParseError;
use super::helper;
use lexer::{Token, TokenType};
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'a> {
    name: &'a str,
    type_params: Vec<&'a str>,
    implements: Vec<&'a str>,
    extends: &'a str,
}

impl<'a> Class<'a> {
    fn new_empty() -> Class<'static> {
        Class {
            name: "",
            type_params: Vec::new(),
            implements: Vec::new(),
            extends: "",
        }
    }
}

/// Given a token stream placed on the 'class' keyword, parse and return a Class. Modifiers will
/// not be parsed, and should be added manually after this function call.
pub fn parse_class<'a>(tok_stream: &mut Iter<'a, Token<'a>>) -> Result<Class<'a>, ParseError> {
    assert_eq!(tok_stream.next().unwrap().val, "class");

    let mut class = Class::new_empty();
    let class_name_tok = try!(tok_stream.next().ok_or(ParseError::new(
        "Expected class name, got EOF".to_owned(),
    )));
    if class_name_tok.token_type != TokenType::Ident {
        return Err(ParseError::new(
            "Expected class name, got '".to_owned() +
                class_name_tok.val + "'",
        ));
    }
    class.name = class_name_tok.val;

    // Parse class start, i.e. the bit with type declarations, extends / implements etc...
    {
        let mut tok = try!(tok_stream.next().ok_or(ParseError::new(
            "Expected token, got EOF".to_owned(),
        )));
        if tok.val == "<" {
            class.type_params = helper::parse_comma_separated_identifier_list(tok_stream);
            let closing = try!(tok_stream.next().ok_or(ParseError::new(
                "Expected '>', got EOF".to_owned(),
            ))).val;
            if closing != ">" {
                return Err(ParseError::new("Expected '>', got EOF".to_owned()));
            }
            tok = try!(tok_stream.next().ok_or(ParseError::new(
                "Expected token, got EOF".to_owned(),
            )));
        }
        if tok.val == "extends" {
            tok = try!(tok_stream.next().ok_or(ParseError::new(
                "Expected token, got EOF".to_owned(),
            )));
            if tok.token_type != TokenType::Ident {
                return Err(ParseError::new("Expected identifier, got ".to_owned() + tok.val));
            }
            class.extends = tok.val;
            tok = try!(tok_stream.next().ok_or(ParseError::new(
                "Expected token, got EOF".to_owned(),
            )));
        }
        if tok.val == "implements" {
            class.implements = helper::parse_comma_separated_identifier_list(tok_stream);
            tok = try!(tok_stream.next().ok_or(ParseError::new(
                "Expected token, got EOF".to_owned(),
            )));
        }
        if tok.val != "{" {
            return Err(ParseError::new("Expected '{'".to_owned()));
        }
    }

    {
        let tok = try!(tok_stream.next().ok_or(ParseError::new(
            "Expected token, got EOF".to_owned(),
        )));
        if tok.val == "}" {
            return Ok(class);
        }
    }

    return Ok(class);
}
