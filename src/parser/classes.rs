use super::*;

#[allow(dead_code)]
pub fn parse_void_method_declarator_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_constructor_declarator_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_generic_method_or_constructor_decl(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_generic_method_or_constructor_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_member_decl(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_method_or_field_decl(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_method_or_field_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_field_declarators_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_class_body_declaration(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_class_body(tokens: &mut TokenIter, src: &str) -> ParseRes {
    assert_term(tokens, src, "{")?;
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        match tok.val(src) {
            "}" => break,
            _ => children.push(parse_class_body_declaration(tokens, src)?),
        }
    }
    assert_term(tokens, src, "}")?;
    Ok(nterm(NTermType::ClassBody, children))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    pub fn test_parse_class_body() {
        let src = "{
            public int foo;
            public ArrayList<String> bar = new ArrayList<>();

            public void someMethod() {
                bar.add(\"Hello!\");
            }
        }";
        let node = parse_class_body(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 5);
    }
}
