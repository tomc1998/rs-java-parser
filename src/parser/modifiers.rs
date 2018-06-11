use super::*;
use parser::annotations::parse_annotation;

pub fn is_modifier_key(s: &str) -> bool {
    s == "public" || s == "protected" || s == "private" ||
        s == "static " || s == "abstract" || s == "final" || s == "native" ||
        s == "synchronized" || s == "transient" || s == "volatile" || s == "strictfp"
}

pub fn is_modifier_or_annot(s: &str) -> bool {
    s == "@" || is_modifier_key(s)
}

#[allow(dead_code)]
pub fn parse_modifier(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let child = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "@" => parse_annotation(tokens, src)?,
        Some(tok) if is_modifier_key(tok.val(src)) => term(*tokens.next().unwrap()),
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
