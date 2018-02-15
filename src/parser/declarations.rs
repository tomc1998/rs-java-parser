//! A parser to parse the declarations of a source file - class / enum / inner class / enum etc.

use super::class::{parse_class};
use super::helper::try_parse_modifier;
use super::ParseError;
use lexer::{TokenType, Token};
use java_model::*;

use std::slice::Iter;

/// Given a token stream, parse all the top level declarations & return them
pub fn parse_top_level_declarations<'a>(
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Result<Vec<Declaration<'a>>, ParseError> {
    println!("Hello");
    let mut declarations = Vec::new();

    loop {
        // Probably no more than 8 modifiers, cba to check
        // TODO: Make this a stack alloc
        // Buffer of modifiers collected whilst parsing class
        let mut modifiers = Vec::with_capacity(8);
        // Advance the iterator until before the next class / enum / interface keyword, store this
        // keyword in val
        let val: String;
        // Consume until we reach type decl
        loop {
            let modifier_opt = try_parse_modifier(tok_stream);
            if modifier_opt.is_some() {
                modifiers.push(modifier_opt.unwrap());
                continue;
            }
            {
                let slice = tok_stream.as_slice();
                if slice.len() == 0 {
                    return Err(ParseError::new("Expected token, found EOF".to_owned()));
                }
                let tok = slice[0];
                if tok.token_type == TokenType::Key &&
                    (tok.val == "class" || tok.val == "enum" || tok.val == "interface")
                {
                    val = tok.val.to_owned();
                    break;
                }
            }
            modifiers.clear();
            tok_stream.next();
        }
        let declaration = match val.as_ref() {
            "class" => {
                let mut class = parse_class(tok_stream)?;
                class.modifiers = modifiers;
                Declaration::Class(class)
            }
            _ => unimplemented!(),
        };

        declarations.push(declaration);

        if tok_stream.as_slice().first().is_none() {
            break;
        }
    }

    return Ok(declarations);
}

#[cfg(test)]
mod tests {
    use lexer::lex_str;
    use parser::class::MemberType;
    use parser::helper::Modifier;
    use super::{parse_top_level_declarations, Declaration};

    #[test]
    fn test() {
        let tokens = lex_str(
            r#"
        public class MyClass {
            public int foo = 4;
            public static double bar;

            protected void doThing() {
                System.out.println("Hello, world");
                if (foo > 4) {
                    doThing();
                }
                else {
                    doOtherThing();
                }
            }

            public static class MyInner {
                private int innerFoo = 0;
            }
        }

        class Foo {
            public int a = 30;
            public int b = 40;
            public Foo(int a, int b) {
                this.a = a;
                this.b = b;
            }
        }"#,
        );
        let declarations = parse_top_level_declarations(&mut tokens.iter());
        if declarations.is_err() {
            panic!(
                "Error parsing top level declarations: {:?}",
                declarations.unwrap_err()
            );
        }
        let declarations = declarations.unwrap();
        assert_eq!(declarations.len(), 2);
        let decl = &declarations[0];
        match decl {
            &Declaration::Class(ref c) => {
                assert_eq!(c.name, "MyClass");
                assert_eq!(c.members.len(), 3);
                assert_eq!(c.modifiers[0], Modifier::Public);
                assert_eq!(c.members[0].name, "foo");
                assert_eq!(c.members[0].member_type, MemberType::Variable);
                assert_eq!(c.members[0].modifiers[0], Modifier::Public);
                assert_eq!(c.members[1].name, "bar");
                assert_eq!(c.members[1].member_type, MemberType::Variable);
                assert_eq!(c.members[1].modifiers[0], Modifier::Public);
                assert_eq!(c.members[1].modifiers[1], Modifier::Static);
                assert_eq!(c.members[2].name, "doThing");
                assert_eq!(c.members[2].member_type, MemberType::Method);
                assert_eq!(c.members[2].modifiers[0], Modifier::Protected);
                assert_eq!(c.inner_classes[0].name, "MyInner");
                assert_eq!(c.inner_classes[0].members[0].name, "innerFoo");
                assert_eq!(c.inner_classes[0].modifiers[0], Modifier::Public);
                assert_eq!(c.inner_classes[0].modifiers[1], Modifier::Static);
            }
        }
        let decl = &declarations[1];
        match decl {
            &Declaration::Class(ref c) => {
                assert_eq!(c.name, "Foo");
                assert_eq!(c.members.len(), 3);
                assert_eq!(c.modifiers.len(), 0);
                assert_eq!(c.members[0].name, "a");
                assert_eq!(c.members[0].member_type, MemberType::Variable);
                assert_eq!(c.members[0].modifiers[0], Modifier::Public);
                assert_eq!(c.members[1].name, "b");
                assert_eq!(c.members[1].member_type, MemberType::Variable);
                assert_eq!(c.members[1].modifiers[0], Modifier::Public);
                assert_eq!(c.members[2].name, "Foo");
                assert_eq!(c.members[2].member_type, MemberType::Constructor);
                assert_eq!(c.members[2].modifiers[0], Modifier::Public);
            }
        }
    }
}
