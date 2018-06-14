use lexer::TokenType;
use super::*;
use super::types::parse_reference_type;
use super::expressions::parse_expression;
use super::identifiers::parse_qualified_identifier;
use super::formal_parameters::parse_variable_declarator_id;
use super::statements::parse_block;
use super::variables::parse_variable_modifier;

#[allow(dead_code)]
pub fn parse_catches(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_catch_clause(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "catch" {
            children.push(parse_catch_clause(tokens, src)?);
        } else { break }
    }
    Ok(nterm(NTermType::Catches, children))
}

#[allow(dead_code)]
pub fn parse_catch_clause(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![
        assert_term(tokens, src, "catch")?,
        assert_term(tokens, src, "(")?];
    while let Some(tok) = tokens.clone().next() {
        match tok.val(src) {
            "final" | "@" => children.push(parse_variable_modifier(tokens, src)?),
            _ => break
        }
    }
    children.push(parse_catch_type(tokens, src)?);
    children.push(assert_term_with_type(tokens, TokenType::Ident)?);
    children.push(assert_term(tokens, src, ")")?);
    children.push(parse_block(tokens, src)?);
    Ok(nterm(NTermType::CatchClause, children))
}

#[allow(dead_code)]
pub fn parse_catch_type(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_qualified_identifier(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "|" {
            tokens.next().unwrap();
            children.push(parse_qualified_identifier(tokens, src)?);
        } else { break }
    }
    Ok(nterm(NTermType::CatchType, children))
}

#[allow(dead_code)]
pub fn parse_finally(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::Finally, vec![assert_term(tokens, src, "finally")?,
                                      parse_block(tokens, src)?]))
}

#[allow(dead_code)]
pub fn parse_resource_specification(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![
        assert_term(tokens, src, "(")?,
        parse_resources(tokens, src)?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == ";" => children.push(term(*tokens.next().unwrap())),
        _ => ()
    }
    children.push(assert_term(tokens, src, ")")?);
    Ok(nterm(NTermType::ResourceSpecification, children))
}

#[allow(dead_code)]
pub fn parse_resources(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_resource(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == ";" {
            tokens.next().unwrap();
            children.push(parse_resource(tokens, src)?);
        } else { break }
    }
    Ok(nterm(NTermType::Resources, children))
}

#[allow(dead_code)]
pub fn parse_resource(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        match tok.val(src) {
            "final" | "@" => children.push(parse_variable_modifier(tokens, src)?),
            _ => break
        }
    }
    children.push(parse_reference_type(tokens, src)?);
    children.push(parse_variable_declarator_id(tokens, src)?);
    children.push(assert_term(tokens, src, "=")?);
    children.push(parse_expression(tokens, src)?);
    Ok(nterm(NTermType::Resource, children))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    pub fn test_parse_catches() {
        let src = "catch (IOException | SocketException e) {
} catch (Exception e) {
}
";
        let node = parse_catches(&mut lex(src, "").unwrap().iter(), src);
        let node = node.unwrap();
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    pub fn test_parse_resource_specification() {
        let src = "(FileInputStream fis = getInputStream(); SomeOtherRes r = someFunc())";
        let node = parse_resource_specification(&mut lex(src, "").unwrap().iter(), src);
        let node = node.unwrap();
        assert_eq!(node.children.len(), 3);
    }
}
