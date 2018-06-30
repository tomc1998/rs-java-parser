use super::*;
use super::statements::parse_block;
use super::modifiers::{is_modifier_or_annot, parse_modifier};

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
pub fn parse_class_body_declaration(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "static" =>
            vec![term(*tokens.next().unwrap()), // "static"
                 parse_block(tokens, src)?],
        Some(tok) if tok.val(src) == "{" => vec![parse_block(tokens, src)?],
        Some(tok) if tok.val(src) == ";" => vec![term(*tokens.next().unwrap())],
        _ => {
            let mut children = Vec::new();
            // Parse modifier list
            while let Some(tok) = tokens.clone().next() {
                if is_modifier_or_annot(tok.val(src)) {
                    children.push(parse_modifier(tokens, src)?);
                } else { break }
            }
            children.push(parse_member_decl(tokens, src)?);
            children
        }
    };
    Ok(nterm(NTermType::ClassBodyDeclaration, children))
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
            static {
                for (int i = 0; i < 100; ++i) {
                    System.out.println(i);
                }
            }

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
