use lexer::TokenType;
use super::*;
use super::types::{parse_non_wildcard_type_arguments,
                   parse_type_arguments_or_diamond};

#[allow(dead_code)]
pub fn parse_created_name(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term_with_type(tokens, TokenType::Ident)?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == ">" => (),
        _ => children.push(parse_type_arguments_or_diamond(tokens, src)?),
    }
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "." {
            tokens.next().unwrap(); // consume '.'
            children.push(assert_term_with_type(tokens, TokenType::Ident)?);
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "}" => break,
                _ => children.push(parse_type_arguments_or_diamond(tokens, src)?),
            }
        } else { break }
    }
    children.push(assert_term(tokens, src, "}")?);
    Ok(nterm(NTermType::CreatedName, children))
}

#[allow(dead_code)]
pub fn parse_array_creator_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_class_creator_rest(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_identifier_suffix(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_selector(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[allow(dead_code)]
pub fn parse_creator(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "<" => vec![
            parse_non_wildcard_type_arguments(tokens, src)?,
            parse_created_name(tokens, src)?,
            parse_class_creator_rest(tokens, src)?],
        _ => vec![
            parse_created_name(tokens, src)?,
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "[" => parse_array_creator_rest(tokens, src)?,
                _ => parse_class_creator_rest(tokens, src)?,
            }]
    };
    Ok(nterm(NTermType::Creator, children))
}
