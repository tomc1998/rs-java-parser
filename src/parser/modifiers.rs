use super::*;
use parser::annotations::parse_annotation;

#[allow(dead_code)]
pub fn parse_modifier(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let child = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "@" => parse_annotation(tokens, src)?,
        Some(tok) if tok.val(src) == "@" ||
            tok.val(src) == "public" ||
            tok.val(src) == "protected" ||
            tok.val(src) == "private" ||
            tok.val(src) == "static " ||
            tok.val(src) == "abstract" ||
            tok.val(src) == "final" ||
            tok.val(src) == "native" ||
            tok.val(src) == "synchronized" ||
            tok.val(src) == "transient" ||
            tok.val(src) == "volatile" ||
            tok.val(src) == "strictfp" => term(*tokens.next().unwrap()),
        Some(tok) => return Err(ParseErr::Point("Expected annotation or modifier".to_owned(), *tok)),
        None => return Err(ParseErr::Raw("Unexpected EOF, expected annotation or modifier".to_owned())),
    };
    Ok(nterm(NTermType::Modifier, vec![child]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    fn test_parse_modifier() {
        let src = "public";
        let node = parse_modifier(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].val(src).unwrap(), "public");

        let src = "@MyAnnotation";
        let node = parse_modifier(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::Annotation) => (),
            _ => panic!("Incorrect ntermtype when parsing an annotation modifier")
        }
    }
}
