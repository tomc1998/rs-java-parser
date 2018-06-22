use super::*;
use super::variables::{parse_variable_modifier};
use super::formal_parameters::parse_variable_declarator_id;
use super::statements::parse_statement_expression;
use super::types::parse_type;
use super::expressions::parse_expression;

/// Parse a 'foreach' control. This assumes the colon has already been spotted
/// in the for control (see comments in parse_for_control).
fn parse_for_var_control(tokens: &mut TokenIter, src: &str) -> ParseRes {
    // Not a strict grammar parse - we aren't parsing both 'forvarcontrol' and
    // 'forvarcontrolrest', just the one 'forvarcontrol' and assuming that the
    // ': Expression' production is chosen.
    let mut children = match tokens.clone().next() {
        Some(tok) if tok.val(src) == "@" || tok.val(src) == "final" =>
            vec![parse_variable_modifier(tokens, src)?],
        _ => vec![],
    };
    children.push(parse_type(tokens, src)?);
    children.push(parse_variable_declarator_id(tokens, src)?);
    children.push(assert_term(tokens, src, ":")?);
    children.push(parse_expression(tokens, src)?);
    Ok(nterm(NTermType::ForVarControl, children))
}

fn parse_for_init(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == ";" => return Ok(nterm(NTermType::ForInit, vec![])),
        _ => (),
    }
    let mut children = vec![parse_statement_expression(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next().unwrap(); // Consume ","
            children.push(parse_statement_expression(tokens, src)?);
        } else { break }
    }
    Ok(nterm(NTermType::ForInit, children))
}

fn parse_for_update(tokens: &mut TokenIter, src: &str) -> ParseRes {
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == ")" => return Ok(nterm(NTermType::ForUpdate, vec![])),
        _ => (),
    }
    let mut children = vec![parse_expression(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next().unwrap(); // Consume ","
            children.push(parse_expression(tokens, src)?);
        } else { break }
    }
    Ok(nterm(NTermType::ForUpdate, children))
}

#[allow(dead_code)]
pub fn parse_for_control(tokens: &mut TokenIter, src: &str) -> ParseRes {
    // So, this one's a little weird to parse. The official grammar is
    // ambiguous, so we need to do some manual bullshit rather than the standard
    // predictive parsing.

    // We're looking ahead to check if there's a ':' token before the first
    // ')' or ';' token. If there is, then this is a 'foreach' loop. Otherwise, this is
    // a standard '3 expression' loop.

    let mut contains_colon = false;
    for t in tokens.clone() {
        match t.val(src) {
            ";" | ")" => break,
            ":" => {
                contains_colon = true;
                break;
            }
            _ => continue
        }
    }

    if contains_colon {
        Ok(nterm(NTermType::ForControl, vec![parse_for_var_control(tokens, src)?]))
    } else {
        Ok(nterm(NTermType::ForControl, vec![
            parse_for_init(tokens, src)?,
            assert_term(tokens, src, ";")?,
            parse_expression(tokens, src)?,
            assert_term(tokens, src, ";")?,
            parse_for_update(tokens, src)?,
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lex;

    #[test]
    fn test_parse_for_var_control() {
        let src = "String s : someStringList";
        let node = parse_for_control(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].children.len(), 4);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::ForVarControl) => (),
            _ => panic!("Incorrect NTermType")
        }
    }

    #[test]
    fn test_parse_for_expr_control() {
        let src = "int ii = 0; ii < someList.len(); ii ++";
        let node = parse_for_control(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 5);
    }
}

