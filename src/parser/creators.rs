use super::*;
use super::types::parse_non_wildcard_type_arguments;

#[allow(dead_code)]
pub fn parse_created_name(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
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
