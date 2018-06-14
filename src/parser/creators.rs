use lexer::TokenType;
use super::*;
use super::variables::parse_array_initializer;
use super::expressions::parse_expression;
use super::classes::parse_class_body;
use super::atoms::{parse_arguments,
                   parse_explicit_generic_invocation_suffix,
                   parse_super_suffix};
use super::types::{is_basic_type,
                   parse_non_wildcard_type_arguments,
                   parse_non_wildcard_type_arguments_or_diamond,
                   parse_type_arguments_or_diamond};

#[allow(dead_code)]
pub fn parse_created_name(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.clone().next() {
        Some(tok) if is_basic_type(tok.val(src)) || tok.token_type == TokenType::Ident => (),
        Some(tok) => return Err(ParseErr::Point("Expected type name".to_owned(), *tok)),
        None => return Err(ParseErr::Raw("Expected type name, got EOF".to_owned())),
    }

    let mut children = vec![term(*tokens.next().unwrap())];

    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "<" =>
            children.push(parse_type_arguments_or_diamond(tokens, src)?),
        _ => ()
    }
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "." {
            tokens.next().unwrap(); // consume '.'
            children.push(assert_term_with_type(tokens, TokenType::Ident)?);
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "<" =>
                    children.push(parse_type_arguments_or_diamond(tokens, src)?),
                _ => (),
            }
        } else { break }
    }
    Ok(nterm(NTermType::CreatedName, children))
}

#[allow(dead_code)]
pub fn parse_class_creator_rest(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_arguments(tokens, src)?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "{" => children.push(parse_class_body(tokens, src)?),
        _ => ()
    }
    Ok(nterm(NTermType::ClassCreatorRest, children))
}

#[allow(dead_code)]
pub fn parse_array_creator_rest(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "[")?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "]" => {
            children.push(term(*tokens.next().unwrap()));
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == "[" {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(assert_term(tokens, src, "]")?);
                } else { break }
            }
            children.push(parse_array_initializer(tokens, src)?);
        }
        _ => {
            children.push(parse_expression(tokens, src)?);
            children.push(assert_term(tokens, src, "]")?);
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == "[" {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(parse_expression(tokens, src)?);
                    children.push(assert_term(tokens, src, "]")?);
                } else { break }
            }
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == "[" {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(assert_term(tokens, src, "]")?);
                } else { break }
            }
        }
    }
    Ok(nterm(NTermType::ArrayCreatorRest, children))
}

#[allow(dead_code)]
pub fn parse_identifier_suffix(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "[" => {
            let mut children = vec![term(*tokens.next().unwrap())];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "." || tok.val(src) == "[" => {
                    while let Some(tok) = tokens.clone().next() {
                        if tok.val(src) == "[" {
                            children.push(term(*tokens.next().unwrap()));
                            children.push(assert_term(tokens, src, "]")?);
                        } else { break }
                    }
                    children.push(assert_term(tokens, src, ".")?);
                    children.push(assert_term(tokens, src, "class")?);
                }
                _ => children.push(parse_expression(tokens, src)?)
            }
            children
        }
        Some(tok) if tok.val(src) == "." => {
            let mut children = vec![term(*tokens.next().unwrap())];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "class" || tok.val(src) == "this" =>
                    children.push(term(*tokens.next().unwrap())),
                Some(tok) if tok.val(src) == "super" => {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(parse_arguments(tokens, src)?);
                }
                Some(tok) if tok.val(src) == "new" => {
                    children.push(term(*tokens.next().unwrap()));
                    match tokens.clone().next() {
                        Some(tok) if tok.val(src) == "<" =>
                            children.push(parse_non_wildcard_type_arguments(tokens, src)?),
                        _ => ()
                    }
                    children.push(parse_inner_creator(tokens, src)?);
                }
                _ => children.push(parse_explicit_generic_invocation(tokens, src)?),
            }
            children
        }
        _ => vec![parse_arguments(tokens, src)?],
    };
    Ok(nterm(NTermType::IdentifierSuffix, children))
}

#[allow(dead_code)]
pub fn parse_explicit_generic_invocation(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::ExplicitGenericInvocationSuffix, vec![
        parse_non_wildcard_type_arguments(tokens, src)?,
        parse_explicit_generic_invocation_suffix(tokens, src)?,
        ]))
}

#[allow(dead_code)]
pub fn parse_inner_creator(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term_with_type(tokens, TokenType::Ident)?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "<" => {
            children.push(parse_non_wildcard_type_arguments_or_diamond(tokens, src)?);
        }
        _ => ()
    }
    children.push(parse_class_creator_rest(tokens, src)?);
    Ok(nterm(NTermType::InnerCreator, children))
}

#[allow(dead_code)]
pub fn parse_selector(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "." => {
            let mut children = vec![term(*tokens.next().unwrap())];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "this" =>
                    children.push(term(*tokens.next().unwrap())),
                Some(tok) if tok.val(src) == "super" => {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(parse_super_suffix(tokens, src)?);
                }
                Some(tok) if tok.val(src) == "new" => {
                    children.push(term(*tokens.next().unwrap()));
                    match tokens.clone().next() {
                        Some(tok) if tok.val(src) == "<" =>
                            children.push(parse_non_wildcard_type_arguments(tokens, src)?),
                        _ => ()
                    }
                    children.push(parse_inner_creator(tokens, src)?);
                }
                Some(tok) if tok.token_type == TokenType::Ident => {
                    children.push(term(*tokens.next().unwrap()));
                    match tokens.clone().next() {
                        Some(tok) if tok.val(src) == "(" =>
                            children.push(parse_arguments(tokens, src)?),
                        _ => ()
                    }
                }
                _ => children.push(parse_explicit_generic_invocation(tokens, src)?),
            }
            children
        },
        _ => vec![
            term(*tokens.next().unwrap()),
            parse_expression(tokens, src)?,
            assert_term(tokens, src, "]")?
        ]
    };
    Ok(nterm(NTermType::Selector, children))
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

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    pub fn test_parse_creator() {
        let src = "Foo(x, y, 1 + 2)";
        let node = parse_creator(&mut lex(src, "").unwrap().iter(), src);
        let node = node.unwrap();
        assert_eq!(node.children.len(), 2);

        let src = "double[x * 32]";
        let node = parse_creator(&mut lex(src, "").unwrap().iter(), src);
        let node = node.unwrap();
        assert_eq!(node.children.len(), 2);
    }
}

