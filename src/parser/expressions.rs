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

pub fn parse_expression(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
}

pub fn parse_expression1(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    unimplemented!()
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
                Some(tok) if tok.token_type == TokenType::Ident => match clone.next() {
                    Some(tok) if tok.val(src) == "<" =>
                        children.push(parse_type(tokens, src)?),
                    _ => children.push(parse_primary(tokens, src)?),
                }
                Some(tok) => return Err(ParseErr::Point("Expected type or expression".to_owned(), *tok)),
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
