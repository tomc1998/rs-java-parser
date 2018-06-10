use super::*;
use super::atoms::parse_primary;
use lexer::TokenType;
use super::types::{is_basic_type, parse_type};
use super::creators::parse_selector;

fn is_prefix_op(s: &str) -> bool {
    s == "++" || s == "--" || s == "!" || s == "~" || s == "+" || s == "-"
}

fn is_postfix_op(s: &str) -> bool {
    s == "++" || s == "--"
}

fn is_infix_op(s: &str) -> bool {
        s == "||" || s == "&&" || s == "|" || s == "^" ||
        s == "&" || s == "==" || s == "!=" || s == "<" ||
        s == ">" || s == "<=" || s == ">=" || s == "<<" ||
        s == ">>" || s == ">>>" || s == "+" || s == "-" ||
        s == "*" || s == "/" || s == "%"
}

fn is_assignment_op(s: &str) -> bool {
    s == "=" || s == "+=" || s == "-=" || s == "*=" || s == "/=" || s == "&=" ||
        s == "|=" || s == "^=" || s == "%=" || s == "<<=" || s == ">>=" || s == ">>>="
}

#[allow(dead_code)]
pub fn parse_prefix_op(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if is_prefix_op(tok.val(src)) => Ok(nterm(NTermType::PrefixOp, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected prefix operator".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected prefix operator, got EOF".to_owned()))
    }
}

#[allow(dead_code)]
pub fn parse_postfix_op(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if is_postfix_op(tok.val(src))
            => Ok(nterm(NTermType::PostfixOp, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected postfix operator".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected postfix operator, got EOF".to_owned()))
    }
}

#[allow(dead_code)]
pub fn parse_infix_op(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if is_infix_op(tok.val(src))
            => Ok(nterm(NTermType::InfixOp, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected operator".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected operator, got EOF".to_owned()))
    }
}

#[allow(dead_code)]
pub fn parse_assignment_op(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.next() {
        Some(tok) if is_assignment_op(tok.val(src))
            => Ok(nterm(NTermType::AssignmentOperator, vec![term(*tok)])),
        Some(tok) => Err(ParseErr::Point("Expected assignment operator".to_owned(), *tok)),
        None => Err(ParseErr::Raw("Expected assignment operator, got EOF".to_owned()))
    }
}

pub fn parse_expression(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_expression1(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if is_assignment_op(tok.val(src)) {
            children.push(parse_assignment_op(tokens, src)?);
            children.push(parse_expression1(tokens, src)?);
        } else { break }
    }
    Ok(nterm(NTermType::Expression, children))
}

pub fn parse_expression1(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_expression2(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "?" {
            children.push(parse_expression1_rest(tokens, src)?)
        } else { break }
    }
    Ok(nterm(NTermType::Expression1, children))
}

#[allow(dead_code)]
pub fn parse_expression1_rest(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::Expression1Rest, vec![
        assert_term(tokens, src, "?")?,
        parse_expression(tokens, src)?,
        assert_term(tokens, src, ":")?,
        parse_expression1(tokens, src)?]))
}


#[allow(dead_code)]
pub fn parse_expression2(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_expression3(tokens, src)?];
    match tokens.clone().next() {
        Some(tok) if is_infix_op(tok.val(src)) ||
            tok.val(src) == "instanceof" =>
            children.push(parse_expression2_rest(tokens, src)?),
        _ => ()
    }
    Ok(nterm(NTermType::Expression2Rest, children))
}

#[allow(dead_code)]
pub fn parse_expression2_rest(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "instanceof" => vec![
            term(*tokens.next().unwrap()),
            parse_type(tokens, src)?],
        _ => {
            let mut children = Vec::new();
            while let Some(tok) = tokens.clone().next() {
                if is_infix_op(tok.val(src)) {
                    children.push(parse_infix_op(tokens, src)?);
                    children.push(parse_expression3(tokens, src)?);
                } else { break }
            }
            children
        }
    };
    Ok(nterm(NTermType::Expression2Rest, children))
}

#[allow(dead_code)]
pub fn parse_expression3(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next() {
        Some(tok) if is_prefix_op(tok.val(src)) => vec![
                parse_prefix_op(tokens, src)?,
                parse_expression3(tokens, src)?],
        Some(tok) if tok.val(src) == "(" => {
            let mut children = vec![term(*tokens.next().unwrap())];
            let mut clone = tokens.clone();
            match clone.next() {
                Some(tok) if is_basic_type(tok.val(src)) =>
                    children.push(parse_type(tokens, src)?),
                Some(tok) if tok.token_type == TokenType::Ident => 
                    children.push(parse_type(tokens, src)?),
                Some(_) => children.push(parse_primary(tokens, src)?),
                None => return Err(ParseErr::Raw("Expected type or expression, got EOF".to_owned())),
            }
            children.push(assert_term(tokens, src, ")")?);
            children.push(parse_expression3(tokens, src)?);
            children
        }
        _ => {
            let mut children = vec![parse_primary(tokens, src)?];
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == "." {
                    children.push(parse_selector(tokens, src)?);
                } else { break }
            }
            while let Some(tok) = tokens.clone().next() {
                if is_postfix_op(tok.val(src)) {
                    children.push(parse_postfix_op(tokens, src)?);
                } else { break }
            }
            children
        }
    };
    Ok(nterm(NTermType::Expression3, children))
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

    #[test]
    fn test_parse_infix_op() {
        let src = ["||", "&&", "|", "^", "&", "==", "!=", "<", ">", "<=", ">=",
                   "<<", ">>", ">>>", "+", "-", "*", "/", "%"];
        assert!(src.iter().all(|src| {
            parse_infix_op(&mut lex(src, "").unwrap().iter(), src).is_ok()
        }));
    }

    #[test]
    fn test_parse_assignment_op() {
        let src = ["=", "+=", "-=", "*=", "/=", "&=", "|=", "^=",
                   "%=", "<<=", ">>=", ">>>="];
        assert!(src.iter().all(|src| {
            parse_assignment_op(&mut lex(src, "").unwrap().iter(), src).is_ok()
        }));
    }

    #[test]
    fn test_parse_expression() {
        let src = "x = y + (float)45 - (float)i++ - 54";
        let node = parse_expression(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
    }

    #[test]
    fn test_parse_expression2() {
        let src = "(float)x + (float)y + 2.0";
        let node = parse_expression2(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);

        let src = "4 + 7 + 234";
        let node = parse_expression2(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);

        let src = "x";
        let node = parse_expression2(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);

        let src = "(Foo)x";
        let node = parse_expression2(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_parse_expression2_rest() {
        let src = "+ foo + bar";
        let node = parse_expression2_rest(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 4);

        let src = "instanceof Foo";
        let node = parse_expression2_rest(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_parse_expression3() {
        let src = "i++";
        let node = parse_expression3(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
        let src = "++i";
        let node = parse_expression3(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
        let src = "-i";
        let node = parse_expression3(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
    }
}
