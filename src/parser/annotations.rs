use super::*;
use parser::identifiers::parse_qualified_identifier;

#[allow(dead_code)]
pub fn parse_annotation_element(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_annotation(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "@")?,
                        parse_qualified_identifier(tokens, src)?];

    let mut clone = tokens.clone();
    match clone.next() {
        Some(tok) if tok.val(src) == "(" => {
            children.push(term(*tokens.next().unwrap()));
            match clone.next() {
                Some(tok) if tok.val(src) == ")" => children.push(term(*tokens.next().unwrap())),
                _ => {
                    children.push(parse_annotation_element(tokens, src)?);
                    children.push(assert_term(tokens, src, ")")?);
                }
            }
        }
        _ => ()
    }

    Ok(nterm(NTermType::Annotation, children))
}

#[allow(dead_code)]
pub fn parse_annotations(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_annotation(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "@" {
            children.push(parse_annotation(tokens, src)?);
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::Annotations, children))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    fn test_parse_annotation() {
        let src = "@MyAnnotation";
        let node = parse_annotation(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].val(src).unwrap(), "@");

        let src = "@MyAnnotation()";
        let node = parse_annotation(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 4);
        assert_eq!(node.children[0].val(src).unwrap(), "@");
        assert_eq!(node.children[2].val(src).unwrap(), "(");
        assert_eq!(node.children[3].val(src).unwrap(), ")");

        let src = "@MyAnnotation(someVal = @Hello)";
        let node = parse_annotation(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 5);
        assert_eq!(node.children[0].val(src).unwrap(), "@");
        assert_eq!(node.children[2].val(src).unwrap(), "(");
        assert_eq!(node.children[4].val(src).unwrap(), ")");
    }

    #[test]
    fn test_parse_annotations() {
        let src = "@MyAnnotation
@OtherAnnotation";
        let node = parse_annotation(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
    }
}
