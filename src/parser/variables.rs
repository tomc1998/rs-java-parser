use super::*;
use super::annotations::parse_annotation;
use super::expressions::parse_expression;
use lexer::TokenType;

pub fn parse_variable_modifier(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let child = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "final" => term(*tokens.next().unwrap()),
        _ => parse_annotation(tokens, src)?,
    };
    Ok(nterm(NTermType::VariableModifier, vec![child]))
}

#[allow(dead_code)]
pub fn parse_array_initializer(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "{")?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "}" => {
            children.push(term(*tokens.next().unwrap()));
            return Ok(nterm(NTermType::ArrayInitializer, children))
        }
        _ => children.push(parse_variable_initializer(tokens, src)?),
    }
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next(); // Skip ','
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "}" => break,
                _ => children.push(parse_variable_initializer(tokens, src)?),
            }
        } else { break; }
    }
    children.push(assert_term(tokens, src, "}")?);
    Ok(nterm(NTermType::ArrayInitializer, children))
}

#[allow(dead_code)]
pub fn parse_variable_initializer(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::VariableInitializer, match tokens.clone().next() {
        Some(tok) if tok.val(src) == "{" => vec![parse_array_initializer(tokens, src)?],
        _ => vec![parse_expression(tokens, src)?],
    }))
}

#[allow(dead_code)]
pub fn parse_variable_declarator_rest(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "[" {
            children.push(term(*tokens.next().unwrap()));
            children.push(assert_term(tokens, src, "]")?);
        } else { break }
    }
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "=" => {
            children.push(term(*tokens.next().unwrap()));
            children.push(parse_variable_initializer(tokens, src)?);
        }
        _ => ()
    }
    Ok(nterm(NTermType::VariableDeclaratorRest, children))
}

#[allow(dead_code)]
pub fn parse_variable_declarator(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::VariableDeclarator,
          vec![assert_term_with_type(tokens, TokenType::Ident)?,
               parse_variable_declarator_rest(tokens, src)?]))
}

#[allow(dead_code)]
pub fn parse_variable_declarators(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_variable_declarator(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next().unwrap(); // Skip ','
            children.push(parse_variable_declarator(tokens, src)?);
        } else { break; }
    }
    Ok(nterm(NTermType::VariableDeclarators, children))
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

    #[test]
    fn test_parse_variable_declarators() {
        let src = "foo = \"hello\", bar = 3, baz = {1, 2, 3}";
        let node = parse_variable_declarators(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
    }
}
