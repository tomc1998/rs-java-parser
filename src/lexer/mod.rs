/// Given a list of tuples where the first element is the input and the 2nd element is the expected
/// lexer token output, check that Lexer::lex(input) produces this correctly.
#[macro_export]
macro_rules! test_lexing {
    ( $(( $input:expr, $expected:expr )),* ) => {
        {
            let lexer = Lexer::new();
            $(
                let mut chars = $input.chars();
                let _tok = lexer.lex(&mut chars)
                    .expect(&("Failed to lex: ".to_owned() + $input));
                assert_eq!($expected, _tok.val);
                assert_eq!(chars.as_str(), &$input[$expected.len()..]);
            )*
        }
    }
}

/// Special version of the test_lexing macro. Performs a double unwrap() on the token returned from
/// the lex() function. This is for lex() functions that return options inside results, like in
/// literal.rs.
#[macro_export]
macro_rules! test_lexing_double_unwrap {
    ( $(( $input:expr, $expected:expr )),* ) => {
        {
            let lexer = Lexer::new();
            $(
                let mut chars = $input.chars();
                let _tok = lexer.lex(&mut chars).unwrap()
                    .expect(&("Failed to lex: ".to_owned() + $input));
                assert_eq!(_tok.val, $expected);
                assert_eq!(chars.as_str(), &$input[$expected.len()..]);
            )*
        }
    }
}

mod token;
mod keywords;
mod identifiers;
mod punctuation;
mod operators;
mod literals;
mod comments;
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

    let keywords_lexer = keywords::KeywordsLexer::new();
    let identifiers_lexer = identifiers::IdentifiersLexer::new();
    let punctuation_lexer = punctuation::PunctuationLexer::new();
    let operators_lexer = operators::OperatorsLexer::new();
    let literals_lexer = literals::LiteralsLexer::new();
    let comments_lexer = comments::CommentsLexer::new();

    while input.as_str().len() > 0 {
        common::consume_whitespace(&mut input);

        if input.as_str().len() <= 0 {
            break;
        }

        let token = comments_lexer.lex(&mut input);
        if token.is_err() {
            panic!("{}", token.err().unwrap());
        }
        let token = token.unwrap();
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }
        let token = literals_lexer.lex(&mut input);
        if token.is_err() {
            panic!("{}", token.err().unwrap());
        }
        let token = token.unwrap();
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }
        let token = keywords_lexer.lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }
        let token = identifiers_lexer.lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }
        let token = punctuation_lexer.lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }
        let token = operators_lexer.lex(&mut input);
        if token.is_some() {
            token_list.push(token.unwrap());
            continue;
        }

        let err = &("Failed to lex: ".to_owned() + input.as_str());
        panic!("{}", err);
    }

    return token_list;
}

 
#[cfg(feature = "bench")]
mod benches {
    extern crate test;
    use super::lex_str;
    use self::test::Bencher;

    #[bench]
    fn test_lex(b: &mut Bencher) {
        let java_code = r#"
        package com.tom.test;

        public class Main {
            public static void main(String[] args) {
                float a = 3.f;
                float b = .2f;
                float c = a + b;
                System.out.println("Hello, world!");
                System.out.println("3 + 0.2 = " + c);
            }
        }
        "#;
        b.iter(|| lex_str(java_code).len());
    }
}
