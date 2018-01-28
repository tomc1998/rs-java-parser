pub enum TokenType {
    Key,
    Punc,
    Ident,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub val: &'a str,
}
