pub enum TokenType {
   Key, Punc, Identifier,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub val: &'a str,
}
