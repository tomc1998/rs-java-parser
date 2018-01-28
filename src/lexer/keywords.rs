use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// Try to lex a keyword from the given char stream. Returns None if not currently placed at a
/// keyword. This will also lex modifiers like public / private.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Option<Token<'a>> {
    let input_str = input.as_str();
    let keyword_list = [
        "import ",
        "package ",
        "class ",
        "if ",
        "while ",
        "for ",
        "public ",
        "private ",
        "static ",
        "final ",
        "synchronized ",
        "native ",
        "strictfp ",
    ];
    for key in keyword_list.iter() {
        if input_str.starts_with(key) {
            let tok = Some(Token {
                token_type: TokenType::Key,
                val: input_str[0..key.len() - 1].trim(),
            });
            input.nth(key.len() - 1);
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
        let mut test_strs = [
            ("package com.tom.test", "package"),
            ("import com.tom.test.MyClass", "import"),
            ("class John", "class"),
            ("if (myBool)", "if"),
            ("while (true)", "while"),
            ("for (;;)", "for"),
            ("public void", "public"),
            ("private int", "private"),
            ("static int", "static"),
            ("final int", "final"),
            ("synchronized void", "synchronized"),
            ("native void", "native"),
            ("strictfp void", "strictfp"),
        ];
        for &(s, tok_val) in test_strs.iter() {
            let mut chars = s.chars();
            let _tok = lex(&mut chars).expect(&("Failed to lex: ".to_owned() + s));
            assert_eq!(tok_val, _tok.val);
            assert_eq!(chars.as_str(), &s[tok_val.len() + 1..]);
        }
    }

    #[test]
    fn it_fails_to_lex_non_java_keywords() {
        let mut test_str_0 = "myClass.doSomething()".chars();
        let tok_0 = lex(&mut test_str_0);
        assert!(tok_0.is_none());
        assert_eq!(test_str_0.as_str(), "myClass.doSomething()");
    }
}
