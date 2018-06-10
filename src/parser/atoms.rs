use super::*;
use super::expressions::parse_expression;
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
pub fn parse_primary(_tokens: TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
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
    }
}
