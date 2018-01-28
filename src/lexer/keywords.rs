use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// Try to lex a keyword from the given char stream. Returns None if not currently placed at a
/// keyword. This will also lex modifiers like public / private.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Option<Token<'a>> {
    let input_str = input.as_str();
    let keyword_list = [
        "import ", "package "
    ];
    for key in keyword_list.iter() {
        if input_str.starts_with(key) {
            let tok = Some(Token {
                token_type: TokenType::Key,
                val: input.as_str()[0..key.len()-1].trim(),
            });
            input.nth(key.len()-1);
            return tok;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn it_lexes_java_keywords() {
        let mut test_str_0 = "package com.tom.test".chars();
        let mut test_str_1 = "import com.tom.test.MyClass".chars();
        let tok_0 = lex(&mut test_str_0).expect("package declaration not lexed");
        let tok_1 = lex(&mut test_str_1).expect("import declaration not lexed");
        assert_eq!(tok_0.val, "package");
        assert_eq!(tok_1.val, "import");
        assert_eq!(test_str_0.as_str(), "com.tom.test");
        assert_eq!(test_str_1.as_str(), "com.tom.test.MyClass");
    }

    #[test]
    fn it_fails_to_lex_non_java_keywords() {
        let mut test_str_0 = "myClass.doSomething()".chars();
        let tok_0 = lex(&mut test_str_0);
        assert!(tok_0.is_none());
        assert_eq!(test_str_0.as_str(), "myClass.doSomething()");
    }
}
