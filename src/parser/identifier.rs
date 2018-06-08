//! Parsing for identifiers

use super::*;
use lexer::{Token, TokenType};

/// A qualified identifier is just a node that contains a list of identifiers.
/// The '.' separating the idents are stripped.
pub fn parse_qualified_identifier(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term_with_type(tokens, TokenType::Ident)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "." {
            tokens.next(); // Skip '.'
            children.push(assert_term_with_type(tokens, TokenType::Ident)?);
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::QualifiedIdentifier, children))
}

pub fn parse_qualified_identifier_list(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_qualified_identifier(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next(); // Skip ','
            children.push(parse_qualified_identifier(tokens, src)?);
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::QualifiedIdentifierList, children))
}

#[cfg(test)]
mod tests {
    use lexer::lex;
    use super::*;

    #[test]
    fn test_parse_qualified_identifier() {
        let src = "com.tom.project.Foo";
        let node = parse_qualified_identifier(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 4);
        assert_eq!(node.children[0].val(src), Some("com"));
        assert_eq!(node.children[1].val(src), Some("tom"));
        assert_eq!(node.children[2].val(src), Some("project"));
        assert_eq!(node.children[3].val(src), Some("Foo"));
    }

    #[test]
    fn test_parse_qualified_identifier_list() {
        let src = "com.tom.project.Foo, com.tom.project.Bar";
        let node = parse_qualified_identifier_list(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children[0].val(src), Some("com"));
        assert_eq!(node.children[0].children[1].val(src), Some("tom"));
        assert_eq!(node.children[0].children[2].val(src), Some("project"));
        assert_eq!(node.children[0].children[3].val(src), Some("Foo"));
        assert_eq!(node.children[1].children[0].val(src), Some("com"));
        assert_eq!(node.children[1].children[1].val(src), Some("tom"));
        assert_eq!(node.children[1].children[2].val(src), Some("project"));
        assert_eq!(node.children[1].children[3].val(src), Some("Bar"));
    }
}
