use super::*;

#[allow(dead_code)]
pub fn parse_prefix_op(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if
            tok.val(src) == "++" ||
            tok.val(src) == "--" ||
            tok.val(src) == "!" ||
            tok.val(src) == "~" ||
            tok.val(src) == "+" ||
            tok.val(src) == "-" => Ok(nterm(NTermType::PrefixOp, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected prefix operator".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected prefix operator, got EOF".to_owned()))
    }
}

#[allow(dead_code)]
pub fn parse_postfix_op(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if
            tok.val(src) == "++" ||
            tok.val(src) == "--" => Ok(nterm(NTermType::PostfixOp, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected postfix operator".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected postfix operator, got EOF".to_owned()))
    }
}

pub fn parse_expression(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

pub fn parse_expression1(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    fn test_parse_prefix_op() {
        let src = ["++", "--", "!", "~", "+", "-"];
        assert!(src.iter().all(|src| {
            parse_prefix_op(&mut lex(src, "").unwrap().iter(), src).is_ok()
        }));
    }

    #[test]
    fn test_parse_postfix_op() {
        let src = ["++", "--"];
        assert!(src.iter().all(|src| {
            parse_postfix_op(&mut lex(src, "").unwrap().iter(), src).is_ok()
        }));
    }
}
