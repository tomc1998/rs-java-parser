use lexer::{Token, TokenType};

mod error;
mod node;
mod identifier;

pub use self::error::*;
pub use self::node::*;

use self::identifier::*;

use std;

type TokenIter<'a> = std::slice::Iter<'a, Token>;
type ParseRes = Result<Node, ParseErr>;

fn parse_compilation_unit(tokens: &mut TokenIter, src: &str) -> ParseRes {
    // Check if this is a control or stmt
    let mut children = Vec::new();
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::CompilationUnit),
        children: children
    })
}

pub fn parse(tokens: &[Token], src: &str) -> ParseRes {
    debug_assert!(!tokens.is_empty());
    parse_compilation_unit(&mut tokens.iter(), src)
}
