use lexer::TokenType;
use super::*;
use super::types::parse_type;
use super::variables::parse_variable_modifier;

#[allow(dead_code)]
pub fn parse_formal_parameters(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "(")?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == ")" => (),
        _ => children.push(parse_formal_parameter_decls(tokens, src)?),
    }
    children.push(assert_term(tokens, src, ")")?);
    Ok(nterm(NTermType::FormalParameters, children))
}

#[allow(dead_code)]
pub fn parse_formal_parameter_decls(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        match tok.val(src) {
            "final" | "@" => children.push(parse_variable_modifier(tokens, src)?),
            _ => break
        }
    }
    children.push(parse_type(tokens, src)?);
    children.push(parse_formal_parameter_decls_rest(tokens, src)?);
    Ok(nterm(NTermType::FormalParameterDecls, children))
}

#[allow(dead_code)]
pub fn parse_formal_parameter_decls_rest(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "..." => vec![
            term(*tokens.next().unwrap()),
            parse_variable_declarator_id(tokens, src)?],
        _ => {
            let mut children = vec![parse_variable_declarator_id(tokens, src)?];
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == "," {
                    tokens.next().unwrap(); // Consume ","
                    children.push(parse_formal_parameter_decls(tokens, src)?);
                } else { break }
            }
            children
        }
    };
    Ok(nterm(NTermType::FormalParameterDeclsRest, children))
}

#[allow(dead_code)]
pub fn parse_variable_declarator_id(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term_with_type(tokens, TokenType::Ident)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "[" {
            children.push(term(*tokens.next().unwrap()));
            children.push(assert_term(tokens, src, "]")?);
        } else { break }
    }
    Ok(nterm(NTermType::VariableDeclaratorId, children))
}

#[cfg(test)]
mod tests {
    use super::*;

    use lexer::lex;

    #[test]
    fn test_parse_formal_parameters() {
        let src = "(int a, int b, Foo<T> someFoo, char[][] charArray)";
        let node = parse_formal_parameters(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
    }
}
