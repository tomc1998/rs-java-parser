use super::*;
use super::expressions::parse_expression;
use super::types::{parse_basic_type, parse_non_wildcard_type_arguments, is_basic_type};
use super::creators::{parse_creator, parse_identifier_suffix};
use lexer::TokenType;

#[allow(dead_code)]
pub fn parse_literal(tokens: &mut TokenIter, _src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if
            tok.token_type == TokenType::NullLit ||
            tok.token_type == TokenType::IntLit ||
            tok.token_type == TokenType::FloatLit ||
            tok.token_type == TokenType::StringLit ||
            tok.token_type == TokenType::CharLit ||
            tok.token_type == TokenType::BoolLit => Ok(nterm(NTermType::Literal, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected literal".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected literal, got EOF".to_owned()))
    }
}

#[allow(dead_code)]
pub fn parse_arguments(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "(")?];
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == ")" => {
            children.push(term(*tokens.next().unwrap()));
            return Ok(nterm(NTermType::Arguments, children));
        }
        _ => children.push(parse_expression(tokens, src)?),
    }

    loop {
        let mut clone = tokens.clone();
        let tok = clone.next();
        match tok {
            Some(tok) if tok.val(src) == ")" => break,
            Some(tok) if tok.val(src) == "," => match clone.next() {
                Some(_) => {
                    tokens.next().unwrap();
                    children.push(parse_expression(tokens, src)?);
                }
                None => return Err(ParseErr::Raw("Expected expression, got EOF".to_owned())),
            }
            None => return Err(ParseErr::Raw("Unexpected EOF in expression".to_owned())),
            Some(tok) => return Err(ParseErr::Point("Expected ')' or ','".to_owned(), *tok)),
        }
    }
    children.push(assert_term(tokens, src, ")")?);
    Ok(nterm(NTermType::ParExpression, children))
}

#[allow(dead_code)]
pub fn parse_par_expression(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "(")?];
    loop {
        let tok = tokens.clone().next();
        match tok {
            Some(tok) if tok.val(src) == ")" => break,
            None => return Err(ParseErr::Raw("Unexpected EOF in expression".to_owned())),
            _ => children.push(parse_expression(tokens, src)?),
        }
    }
    children.push(assert_term(tokens, src, ")")?);
    Ok(nterm(NTermType::ParExpression, children))
}

#[allow(dead_code)]
pub fn parse_super_suffix(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "." => {
            let mut children = vec![
                term(*tokens.next().unwrap()),
                assert_term_with_type(tokens, TokenType::Ident)?];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "(" => {
                    children.push(parse_arguments(tokens, src)?);
                }
                _ => ()
            }
            children
        }
        _ => vec![parse_arguments(tokens, src)?]
    };
    Ok(nterm(NTermType::SuperSuffix, children))
}

#[allow(dead_code)]
pub fn parse_explicit_generic_invocation_suffix(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "super" => vec![
            term(*tokens.next().unwrap()),
            parse_super_suffix(tokens, src)?],
        Some(tok) if tok.token_type == TokenType::Ident => vec![
            term(*tokens.next().unwrap()),
            parse_arguments(tokens, src)?],
        Some(tok) => return Err(ParseErr::Point("Expected 'super' or identifier".to_owned(), *tok)),
        None => return Err(ParseErr::Raw("Expected 'super' or identifier, got EOF".to_owned())),
    };
    Ok(nterm(NTermType::ExplicitGenericInvocationSuffix, children))
}

#[allow(dead_code)]
pub fn parse_primary(tokens: &mut TokenIter, src: &str) -> ParseRes {
    // type=lit   Literal
    // valu=(     ParExpression
    // valu=this  this [Arguments]
    // valu=super super SuperSuffix
    // valu=new   new Creator
    // valu=<     NonWildcardTypeArguments (ExplicitGenericInvocationSuffix | this Arguments)
    // type=ident Identifier { . Identifier } [IdentifierSuffix]
    // valu=bt    BasicType {[]} . class
    // valu=void  void . class
    let children = match tokens.clone().next() {
        Some(tok) if tok.is_literal() => vec![term(*tokens.next().unwrap())],
        Some(tok) if tok.val(src) == "(" => vec![parse_par_expression(tokens, src)?],
        Some(tok) if tok.val(src) == "this" => {
            let mut children = vec![term(*tokens.next().unwrap())];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "(" => children.push(parse_arguments(tokens, src)?),
                _ => ()
            }
            children
        }
        Some(tok) if tok.val(src) == "super" => vec![
            term(*tokens.next().unwrap()),
            parse_super_suffix(tokens, src)?],
        Some(tok) if tok.val(src) == "new" => vec![
            term(*tokens.next().unwrap()),
            parse_creator(tokens, src)?],
        Some(tok) if tok.val(src) == "<" => {
            let mut children = vec![parse_non_wildcard_type_arguments(tokens, src)?];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "this" => {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(parse_arguments(tokens, src)?);
                }
                _ => children.push(parse_explicit_generic_invocation_suffix(tokens, src)?)
            }
            children
        }
        Some(tok) if tok.token_type == TokenType::Ident => {
            let mut children = vec![term(*tokens.next().unwrap())];
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == "." {
                    tokens.next().unwrap();
                    children.push(assert_term_with_type(tokens, TokenType::Ident)?);
                } else {
                    break
                }
            }
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "(" || tok.val(src) == "." =>
                    children.push(parse_identifier_suffix(tokens, src)?),
                _ => ()
            }
            children
        }
        Some(tok) if is_basic_type(tok.val(src)) => {
                let mut children = vec![parse_basic_type(tokens, src)?];
                // Consume all []
                let mut consumed = 0;
                let mut clone = tokens.clone();
                while let Some(tok) = clone.next() {
                    if tok.val(src) == "[" {
                        if let Some(tok) = clone.next() {
                            if tok.val(src) == "]" {
                                consumed += 2
                            } else {
                                return Err(ParseErr::Point("Mismatched []".to_owned(), *tok));
                            }
                        } else {
                            return Err(ParseErr::Point("Mismatched []".to_owned(), *tok));
                        }
                    } else {
                        break;
                    }
                }
                for _ in 0..consumed { children.push(term(*tokens.next().unwrap())); }
                children.push(assert_term(tokens, src, ".")?);
                children.push(assert_term(tokens, src, "class")?);
                children
            }
        Some(tok) if tok.val(src) == "void" =>
            vec![
                assert_term(tokens, src, "void")?,
                assert_term(tokens, src, ".")?,
                assert_term(tokens, src, "class")?
            ],
        Some(tok) => return Err(ParseErr::Point("Expected type, literal, or value".to_owned(), *tok)),
        None => return Err(ParseErr::Raw("Expected type, literal, or value, got EOF".to_owned())),
    };
    Ok(nterm(NTermType::Primary, children))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;


    #[test]
    fn test_parse_literal() {
        let src = ["24.0", "24", "null", "\"Hello\"", "'a'", "true"];
        assert!(src.iter().all(|src| {
            parse_literal(&mut lex(src, "").unwrap().iter(), src).is_ok()
        }));
    }

    #[test]
    fn test_parse_par_expression() {
        let src = "()";
        let node = parse_par_expression(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);

        let src = "(foo + bar)";
        let node = parse_par_expression(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
    }

    #[test]
    fn test_parse_arguments() {
        let src = "()";
        let node = parse_arguments(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);

        let src = "(foo, bar, foo + bar)";
        let node = parse_arguments(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 5);
    }

    #[test]
    fn test_parse_super_suffix() {
        let src = ".foo";
        let node = parse_super_suffix(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);

        let src = ".foo()";
        let node = parse_super_suffix(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);

        let src = "()";
        let node = parse_super_suffix(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);

        let src = "(foo, bar)";
        let node = parse_super_suffix(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_parse_explicit_generic_invocation_suffix() {
        let src = "super.foo()";
        let node = parse_explicit_generic_invocation_suffix(&mut lex(src, "").unwrap().iter(),
                                                            src).unwrap();
        assert_eq!(node.children.len(), 2);

        let src = "foo()";
        let node = parse_explicit_generic_invocation_suffix(&mut lex(src, "").unwrap().iter(),
                                                            src).unwrap();
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_parse_primary() {
        let src = "1.0";
        let node = parse_primary(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);

        let src = "boolean.class";
        let node = parse_primary(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
    }
}
