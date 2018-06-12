use super::*;
use super::annotations::parse_annotation;

pub fn parse_variable_modifier(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let child = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "final" => term(*tokens.next().unwrap()),
        _ => parse_annotation(tokens, src)?,
    };
    Ok(nterm(NTermType::VariableModifier, vec![child]))
}

#[allow(dead_code)]
pub fn parse_variable_declarators(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    fn test_parse_variable_modifier() {
        let src = "@MyAnnotation";
        let node = parse_variable_modifier(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::Annotation) => (),
            _ => panic!("Wrong nterm type"),
        }

        let src = "final";
        let node = parse_variable_modifier(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].val(src), Some("final"));
    }
}
