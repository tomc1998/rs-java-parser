use super::*;
use super::compilation_units::parse_class_or_interface_declaration;
use lexer::TokenType;
use super::atoms::parse_par_expression;
use super::switches::parse_switch_block_statement_groups;
use super::expressions::parse_expression;
use super::for_loops::parse_for_control;
use super::modifiers::is_modifier_or_annot;
use super::types::{is_basic_type, parse_type};
use super::try_catches::{parse_resource_specification,
                         parse_catches,
                         parse_finally};
use super::variables::{parse_variable_modifier, parse_variable_declarators};

pub fn is_variable_modifier(s: &str) -> bool {
    s == "final" || s == "@"
}

#[allow(dead_code)]
pub fn parse_local_variable_declaration_statement(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        if is_variable_modifier(tok.val(src)) {
            children.push(parse_variable_modifier(tokens, src)?);
        } else { break }
    }
    children.push(parse_type(tokens, src)?);
    children.push(parse_variable_declarators(tokens, src)?);
    children.push(assert_term(tokens, src, ";")?);
    Ok(nterm(NTermType::LocalVariableDeclarationStatement, children))
}

pub fn parse_block_statement(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    let children = match clone.next() {
        Some(tok) if
        is_modifier_or_annot(tok.val(src))
            || tok.val(src) == "class"
            || tok.val(src) == "interface"
            || tok.val(src) == "enum"
            => vec![parse_class_or_interface_declaration(tokens, src)?],
        Some(tok) if tok.token_type == TokenType::Ident => match clone.next() {
            Some(tok) if tok.val(src) == ":" => vec![parse_statement(tokens, src)?],
            _ => vec![parse_local_variable_declaration_statement(tokens, src)?],
        }
        Some(tok) if is_variable_modifier(tok.val(src)) ||
            is_basic_type(tok.val(src)) => vec![
                parse_local_variable_declaration_statement(tokens, src)?],
        _ => vec![parse_statement(tokens, src)?],
    };
    Ok(nterm(NTermType::BlockStatement, children))
}

#[allow(dead_code)]
pub fn parse_block_statements(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = Vec::new();
    loop {
        match tokens.clone().next() {
            Some(tok) if tok.val(src) == "}" => break,
            _ => children.push(parse_block_statement(tokens, src)?),
        }
    }
    Ok(nterm(NTermType::BlockStatements, children))
}

#[allow(dead_code)]
pub fn parse_block(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::Block,
          vec![assert_term(tokens, src, "{")?,
               parse_block_statements(tokens, src)?,
               assert_term(tokens, src, "}")?]))
}

#[allow(dead_code)]
pub fn parse_statement(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    let children = match clone.next() {
        Some(tok) if tok.val(src) == "{" => vec![parse_block(tokens, src)?],
        Some(tok) if tok.val(src) == ";" => vec![term(*tokens.next().unwrap())],
        Some(tok) if tok.token_type == TokenType::Ident => vec![
            term(*tokens.next().unwrap()),
            assert_term(tokens, src, ":")?,
            parse_statement(tokens, src)?],
        Some(tok) if tok.val(src) == "if" => {
            let mut children = vec![
            term(*tokens.next().unwrap()),
            parse_par_expression(tokens, src)?,
            parse_statement(tokens, src)?];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "else" => {
                    children.push(term(*tokens.next().unwrap()));
                    children.push(parse_statement(tokens, src)?);
                }
                _ => ()
            }
            children
        }
        Some(tok) if tok.val(src) == "assert" => {
            let mut children = vec![term(*tokens.next().unwrap()),
                                    parse_expression(tokens, src)?];
            while let Some(tok) = tokens.clone().next() {
                if tok.val(src) == ":" {
                    tokens.next().unwrap();
                    children.push(parse_expression(tokens, src)?);
                } else { break }
            }
            children
        },
        Some(tok) if tok.val(src) == "switch" => vec![
            term(*tokens.next().unwrap()),
            parse_par_expression(tokens, src)?,
            assert_term(tokens, src, "{")?,
            parse_switch_block_statement_groups(tokens, src)?,
            assert_term(tokens, src, "}")?],
        Some(tok) if tok.val(src) == "while" => vec![
            term(*tokens.next().unwrap()),
            parse_par_expression(tokens, src)?,
            parse_statement(tokens, src)?,
            ],
        Some(tok) if tok.val(src) == "do" => vec![
            term(*tokens.next().unwrap()),
            parse_statement(tokens, src)?,
            assert_term(tokens, src, "while")?,
            parse_par_expression(tokens, src)?,
            assert_term(tokens, src, ";")?],
        Some(tok) if tok.val(src) == "for" => vec![
            term(*tokens.next().unwrap()),
            assert_term(tokens, src, "(")?,
            parse_for_control(tokens, src)?,
            assert_term(tokens, src, ")")?,
            parse_statement(tokens, src)?],
        Some(tok) if tok.val(src) == "break" || tok.val(src) == "continue" => {
            let mut children = vec![term(*tokens.next().unwrap())];
            match tokens.clone().next() {
                Some(tok) if tok.token_type == TokenType::Ident => {
                    children.push(term(*tokens.next().unwrap()));
                }
                _ => ()
            }
            children.push(assert_term(tokens, src, ";")?);
            children
        }
        Some(tok) if tok.val(src) == "return" => {
            let mut children = vec![term(*tokens.next().unwrap())];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) != ";" => {
                    children.push(parse_expression(tokens, src)?);
                }
                _ => ()
            }
            children.push(assert_term(tokens, src, ";")?);
            children
        }
        Some(tok) if tok.val(src) == "throw" => vec![
            term(*tokens.next().unwrap()),
            parse_expression(tokens, src)?],
        Some(tok) if tok.val(src) == "synchronized" => vec![
            term(*tokens.next().unwrap()),
            parse_par_expression(tokens, src)?,
            parse_block(tokens, src)?,
            ],
        Some(tok) if tok.val(src) == "try" => {
            // FIXME: So, this is actually incorrect parsing. Here is the official grammar:
            // try Block (Catches | [Catches] Finally)
            // try ResourceSpecification Block [Catches] [Finally]
            // I.e. Catches can only be not present if finally is present in the
            // non-resource-specified block. BUT, for my personal uses, I don't
            // need to make this distinction. This is much more simple parsing
            // code, if slightly incorrect, which always allows Catches to be
            // optional:
            let mut children = vec![term(*tokens.next().unwrap()),
                                match clone.next() {
                                    Some(tok) if tok.val(src) == "{" => parse_block(tokens, src)?,
                                    _ => parse_resource_specification(tokens, src)?,
                                }];
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "catch" => children.push(parse_catches(tokens, src)?),
                _ => ()
            }
            match tokens.clone().next() {
                Some(tok) if tok.val(src) == "finally" => children.push(parse_finally(tokens, src)?),
                _ => ()
            }
            children
        }
        Some(_) => vec![parse_expression(tokens, src)?],
        None => return Err(ParseErr::Raw("Expected statement, found EOF".to_owned())),
    };
    Ok(nterm(NTermType::Statement, children))
}

#[cfg(test)]
mod tests {
    use super::*;

    use lexer::lex;

    #[test]
    fn test_parse_block() {
        let src = "{
    Foo f = new Foo();
    float f = 0.0;
    String s0 = \"Hello, \", s1 = \"world!\";
    String hello = s0 + s1;
}";
        let node = parse_block(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[1].children.len(), 4);
    }
}
