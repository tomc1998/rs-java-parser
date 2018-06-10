/// An index into some source code
#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Hash)]
pub struct Point(pub usize);

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum TokenType {
    Ident, Punc, Key, Op,
    IntLit, FloatLit, StringLit, CharLit, BoolLit, NullLit,
    Comment
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Token {
    pub start: Point,
    pub end: Point,
    pub token_type: TokenType,
}

impl<'a> Token {
    pub fn new_ident(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Ident }
    }
    pub fn new_punc(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Punc }
    }
    pub fn new_key(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Key }
    }
    pub fn new_op(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Op }
    }
    pub fn new_float_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::FloatLit }
    }
    pub fn new_int_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::IntLit }
    }
    pub fn new_null_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::NullLit }
    }
    pub fn new_char_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::CharLit }
    }
    pub fn new_string_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::StringLit }
    }
    pub fn new_bool_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::BoolLit }
    }
    pub fn new_comment(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Comment }
    }

    pub fn val(&self, src: &'a str) -> &'a str {
        unsafe {
            src.slice_unchecked(self.start.0, self.end.0)
        }
    }

    pub fn is_literal(&self) -> bool {
        self.token_type == TokenType::IntLit ||
            self.token_type == TokenType::FloatLit ||
            self.token_type == TokenType::StringLit ||
            self.token_type == TokenType::CharLit ||
            self.token_type == TokenType::BoolLit ||
            self.token_type == TokenType::NullLit
    }
}
