//! Module for parsing class declarations

use super::ParseError;
use super::helper::{self, parse_modifiers, consume_surrounded, Modifier};
use lexer::{Token, TokenType};
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberType {
    Variable,
    Method,
    Constructor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMember<'a> {
    pub modifiers: Vec<Modifier>,
    pub name: &'a str,
    pub member_type: MemberType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'a> {
    pub name: &'a str,
    pub type_params: Vec<&'a str>,
    pub implements: Vec<&'a str>,
    pub extends: &'a str,
    pub members: Vec<ClassMember<'a>>,
}

impl<'a> Class<'a> {
    fn new_empty() -> Class<'static> {
        Class {
            name: "",
            type_params: Vec::new(),
            implements: Vec::new(),
            extends: "",
            members: Vec::new(),
        }
    }
}

/// Parse a class member. Token stream should be on the class member (i.e. just after the {, or
/// just after another class member)
///
/// # Params
/// * `class_name` - This is needed to detect constructors.
pub fn parse_class_member<'a>(
    class_name: &'a str,
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Result<ClassMember<'a>, ParseError> {
    let modifiers = try!(parse_modifiers(tok_stream));

    let tok = try!(tok_stream.next().ok_or(ParseError::new(
        "Expected token, got EOF".to_owned(),
    )));

    // If constructor, just parse straight away
    if tok.val == class_name {
        // Consume until we hit the start of the method
        loop {
            let tok = try!(tok_stream.next().ok_or(ParseError::new(
                "Expected token, got EOF".to_owned(),
            )));
            if tok.val == ")" {
                break;
            }
        }
        // Consume all in {}
        return Ok(ClassMember {
            modifiers: modifiers,
            name: tok.val,
            member_type: MemberType::Constructor,
        });
    }

    // Either a method or a variable, regardless this ident will be the type
    if tok.token_type != TokenType::Ident && tok.token_type != TokenType::Key {
        return Err(ParseError::new(
            "Expected identifier, got ".to_owned() + tok.val,
        ));
    }
    let member_java_type = tok.val;

    // This will be the member name
    let tok = try!(tok_stream.next().ok_or(ParseError::new(
        "Expected member name, got EOF".to_owned(),
    )));

    let member_name = tok.val;

    // If this is a ; or =, then it's a variable - if it's a (, it's a method. If it's something
    // else, return an error.
    let tok = try!(tok_stream.as_slice().first().ok_or(ParseError::new(
        "Expected token, got EOF".to_owned(),
    )));
    match tok.val {
        ";" | "=" => {
            // Variable
            // Consume until we hit ;
            loop {
                let tok = try!(tok_stream.next().ok_or(ParseError::new(
                    "Expected token, got EOF".to_owned(),
                )));
                if tok.val == ";" {
                    break;
                }
            }
            return Ok(ClassMember {
                modifiers: modifiers,
                name: member_name,
                member_type: MemberType::Variable,
            });
        }
        "(" => {
            // Method
            // Now we need to consume until we hit the matching }
            try!(consume_surrounded(tok_stream, "(", ")"));
            try!(consume_surrounded(tok_stream, "{", "}"));
            return Ok(ClassMember {
                modifiers: modifiers,
                name: member_name,
                member_type: MemberType::Method,
            });
        }
        _ => {
            return Err(ParseError::new(
                "Expected ';', '=', or '(', got ".to_owned() + tok.val,
            ));
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
                return Err(ParseError::new(
                    "Expected identifier, got ".to_owned() + tok.val,
                ));
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

    loop {
        println!("Hello");
        let tok = try!(tok_stream.as_slice().first().ok_or(ParseError::new(
            "Expected token, got EOF".to_owned(),
        )));
        if tok.val == "}" {
            // Make sure to consume this token we peeked
            tok_stream.next().unwrap();
            return Ok(class);
        } else {
            class.members.push(try!(
                parse_class_member(class.name, tok_stream)
            ));
        }
    }
}
