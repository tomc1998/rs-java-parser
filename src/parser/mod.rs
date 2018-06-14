use lexer::Token;

mod error;
mod node;
mod util;
mod identifiers;
mod modifiers;
mod annotations;
mod imports;
mod types;
mod creators;
mod expressions;
mod switches;
mod statements;
mod for_loops;
mod try_catches;
mod atoms;
mod compilation_units;
mod classes;
mod variables;
mod formal_parameters;

pub use self::error::*;
pub use self::node::*;

use std;

type TokenIter<'a> = std::slice::Iter<'a, Token>;
type ParseRes = Result<Node, ParseErr>;

fn parse_compilation_unit(_tokens: &mut TokenIter, _src: &str) -> ParseRes {
    // Check if this is a control or stmt
    let children = Vec::new();
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::CompilationUnit),
        children: children
    })
}

pub fn parse(tokens: &[Token], src: &str) -> ParseRes {
    debug_assert!(!tokens.is_empty());
    parse_compilation_unit(&mut tokens.iter(), src)
}
