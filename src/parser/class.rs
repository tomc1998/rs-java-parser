//! Module for parsing class declarations

use super::ParseError;
use super::helper::{self, parse_modifiers, consume_surrounded};
use lexer::{Token, TokenType};
use std::slice::Iter;
use java_model::*;

/// Either a classmember or inner class. Used by parse_class_member, which needs to return one of
/// these two values.
enum ClassMemberOrInnerClass<'a> {
    Class(Class<'a>),
    ClassMember(ClassMember<'a>),
}

/// Parse a class member. Token stream should be on the class member (i.e. just after the {, or
/// just after another class member)
///
/// # Params
/// * `class_name` - This is needed to detect constructors.
/// # Returns
/// Can return either a class member or an inner class.
fn parse_class_member<'a>(
    class_name: &'a str,
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Result<ClassMemberOrInnerClass<'a>, ParseError> {
    let modifiers = try!(parse_modifiers(tok_stream));

    let tok = try!(tok_stream.as_slice().first().ok_or(ParseError::new(
        "Expected token, got EOF".to_owned(),
    )));

    if tok.val == "class" {
        let mut class = parse_class(tok_stream)?;
        class.modifiers = modifiers;
        return Ok(ClassMemberOrInnerClass::Class(class));
    }

    tok_stream.next().unwrap();

    // If constructor, just parse straight away
    if tok.val == class_name {
        // Consume until we hit the start of the method
        try!(consume_surrounded(tok_stream, "(", ")"));
        try!(consume_surrounded(tok_stream, "{", "}"));
        // Consume all in {}
        return Ok(ClassMemberOrInnerClass::ClassMember(ClassMember {
            modifiers: modifiers,
            name: tok.val,
            member_type: MemberType::Constructor,
        }));
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
            return Ok(ClassMemberOrInnerClass::ClassMember(ClassMember {
                modifiers: modifiers,
                name: member_name,
                member_type: MemberType::Variable,
            }));
        }
        "(" => {
            // Method
            // Now we need to consume until we hit the matching }
            try!(consume_surrounded(tok_stream, "(", ")"));
            try!(consume_surrounded(tok_stream, "{", "}"));
            return Ok(ClassMemberOrInnerClass::ClassMember(ClassMember {
                modifiers: modifiers,
                name: member_name,
                member_type: MemberType::Method,
            }));
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
        } else if tok.val == "class" {
            // If inner class, parse a class.
        } else {
            let member_or_inner = try!(parse_class_member(class.name, tok_stream));
            match member_or_inner {
                ClassMemberOrInnerClass::Class(c) => class.inner_classes.push(c),
                ClassMemberOrInnerClass::ClassMember(m) => class.members.push(m),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_class;
    use super::helper::Modifier;
    use lexer::lex_str;

    #[test]
    fn test_class_parse() {
        let class_src = r#"
        class Foo {
            public static void main(String[] args) {
                System.out.println("hello, world");
            }
        }
        "#;
        let lexed = lex_str(class_src);
        let class = parse_class(&mut lexed.iter()).expect("Class failed to parse");
        assert_eq!(class.name, "Foo");
        assert_eq!(class.members.len(), 1);
        assert_eq!(class.members[0].modifiers[0], Modifier::Public);
        assert_eq!(class.members[0].modifiers[1], Modifier::Static);
        assert_eq!(class.members[0].name, "main");
    }
}
