use super::*;
use parser::identifiers::parse_qualified_identifier;
use super::expressions::parse_expression1;
use lexer::TokenType;

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

#[allow(dead_code)]
pub fn parse_element_value_pair(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::ElementValuePair, vec![
        assert_term_with_type(tokens, TokenType::Ident)?,
        assert_term(tokens, src, "=")?,
        parse_element_value(tokens, src)?,
        ]))
}

#[allow(dead_code)]
pub fn parse_element_value_pairs(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_element_value_pair(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next(); // Skip ','
            children.push(parse_element_value_pair(tokens, src)?);
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::ElementValuePairs, children))
}

#[allow(dead_code)]
pub fn parse_element_values(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_element_value(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next(); // Skip ','
            children.push(parse_element_value(tokens, src)?);
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::ElementValues, children))
}

#[allow(dead_code)]
pub fn parse_element_value_array_initializer(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "{")?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "}" => children.push(term(*tokens.next().unwrap())),
        _ => children.push(parse_element_values(tokens, src)?),
    }
    Ok(nterm(NTermType::ElementValueArrayInitializer, children))
}

#[allow(dead_code)]
pub fn parse_element_value(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    Ok(nterm(NTermType::ElementValue, vec![match clone.next() {
        Some(tok) if tok.val(src) == "@" => parse_annotation(tokens, src)?,
        Some(tok) if tok.val(src) == "{" => parse_element_value_array_initializer(tokens, src)?,
        _ => parse_expression1(tokens, src)?,
    }]))
}

#[allow(dead_code)]
pub fn parse_annotation_element(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    let child = match clone.next() {
        Some(tok) if tok.token_type == TokenType::Ident => match clone.next() {
            Some(tok) if tok.val(src) == "=" =>
                parse_element_value_pairs(tokens, src)?,
            _ => parse_element_value(tokens, src)?,
        }
        _ => parse_element_value(tokens, src)?,
    };
    Ok(nterm(NTermType::AnnotationElement, vec![child]))
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

    #[test]
    fn test_parse_annotation_element() {
        let src = "ident = {@Annot, @OtherAnnot}";
        let node = parse_annotation_element(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::ElementValuePairs) => (),
            _ => panic!("Wrong nterm type"),
        }

        let src = "{@Annot, @OtherAnnot}";
        let node = parse_annotation_element(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::ElementValue) => (),
            _ => panic!("Wrong nterm type"),
        }
    }
}
