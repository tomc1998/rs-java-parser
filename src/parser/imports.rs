use super::*;
use super::util::consume_maybe;
use super::identifiers::parse_qualified_identifier;
use lexer::TokenType;

#[allow(dead_code)]
pub fn parse_import(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![
        assert_term(tokens, src, "import")?,
    ];
    if let Some(tok) = consume_maybe(tokens, src, "static", TokenType::Key) {
        children.push(term(tok));
    }
    children.push(parse_qualified_identifier(tokens, src)?);

    // Add [.*]
    let mut clone = tokens.clone();
    match clone.next() {
        Some(tok) if tok.val(src) == "." => match clone.next() {
            Some(tok) if tok.val(src) == "*" => {
                children.push(term(*tokens.next().unwrap()));
                children.push(term(*tokens.next().unwrap()));
            }
            _ => (),
        }
        _ => (),
    }

    children.push(assert_term(tokens, src, ";")?);

    Ok(nterm(NTermType::QualifiedIdentifier, children))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    fn test_parse_import() {
        let src = "import com.tom.project.Foo;";
        let node = parse_import(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children[0].val(src), Some("import"));
        assert_eq!(node.children[2].val(src), Some(";"));
    }

    #[test]
    fn test_parse_static_import() {
        let src = "import static com.tom.project.Foo;";
        let node = parse_import(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 4);
        assert_eq!(node.children[0].val(src), Some("import"));
        assert_eq!(node.children[1].val(src), Some("static"));
        assert_eq!(node.children[3].val(src), Some(";"));
    }

}
