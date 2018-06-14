use lexer::TokenType;
use super::*;
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
pub fn parse_catch_type(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_finally(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_resource_specification(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}
