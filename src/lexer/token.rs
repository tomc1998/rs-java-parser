#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TokenType {
    Key,
    Punc,
    Ident,
    Literal,
    Operator,
    Comment,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub val: &'a str,
}
