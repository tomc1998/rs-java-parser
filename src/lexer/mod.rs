mod token;
mod keywords;
mod identifiers;
mod punctuation;
mod literal;
mod common;

pub use self::token::{Token, TokenType};

use std;

pub type CharStream<'a> = std::str::Chars<'a>;

/// Lex a given source file, and return a list of tokens.
pub fn lex_str<'a>(input: &'a str) -> Vec<Token<'a>> {
    lex_char_stream(input.chars())
}

/// Lex a given source file, and return a list of tokens.
pub fn lex_char_stream<'a>(mut input: CharStream<'a>) -> Vec<Token<'a>> {
    let mut token_list = Vec::new();

    while input.as_str().len() > 0 {
        common::consume_whitespace(&mut input);

        if input.as_str().len() <= 0 { break; }

        let token = punctuation::lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }

        let token = literal::lex(&mut input);
        if token.is_err() {
            panic!("{}", token.err().unwrap());
        }
        let token = token.unwrap();
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }

        let token = keywords::lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }

        let token = identifiers::lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }

        let err = &("Failed to lex: ".to_owned() + input.as_str());
        panic!("{}", err);
    }

    return token_list;
}

#[cfg(test)]
mod test {
    use super::lex_str;
    #[test]
    fn it_should_lex_valid_java_code() {
        let java_code = r#"
        package com.tom.test;

        public class Main {
            public static void main(String[] args) {
                System.out.println("Hello, world!");
            }
        }
        "#;
        println!("{:?}", lex_str(java_code));
    }
}
