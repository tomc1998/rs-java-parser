//! A parser to parse the declarations of a source file - class / enum / inner class / enum etc.

use super::class::{Class, parse_class};
use super::ParseError;
use lexer::{TokenType, Token};

use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<'a> {
    Class(Class<'a>),
}

/// Given a token stream, parse all the top level declarations & return them
pub fn parse_top_level_declarations<'a>(
    tok_stream: &mut Iter<'a, Token<'a>>,
) -> Result<Vec<Declaration<'a>>, ParseError> {
    let mut declarations = Vec::new();

    // Advance the iterator until before the next class / enum / interface keyword, store this
    // keyword in val
    let val: String;
    // Consume until we reach type decl
    loop {
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
        tok_stream.next();
    }
    let declaration = match val.as_ref() {
        "class" => Declaration::Class(try!(parse_class(tok_stream))),
        _ => unimplemented!(),
    };

    declarations.push(declaration);

    return Ok(declarations);
}

#[cfg(test)]
mod tests {
    use lexer::lex_str;
    use super::parse_top_level_declarations;

    #[test]
    fn test() {
        let tokens = lex_str("hello void class Hello { asd }");
        let declarations = parse_top_level_declarations(&mut tokens.iter());
        if declarations.is_err() {
            panic!(
                "Error parsing top level declarations: {:?}",
                declarations.unwrap_err()
            );
        }
    }
}
