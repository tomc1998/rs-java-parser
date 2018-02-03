use regex::Regex;
use lexer::{CharStream, Token, TokenType};

pub struct KeywordsLexer {
    re_keyword: Regex,
}

impl KeywordsLexer {
    pub fn new() -> KeywordsLexer {
        KeywordsLexer {
            re_keyword: Regex::new(r"^(abstract|continue|for|new|switch|assert|default|goto|package|synchronized|boolean|do|if|private|this|break|double|implements|protected|throw|byte|else|import|public|throws|case|enum|instanceof|return|transient|catch|extends|int|short|try|char|final|interface|static|void|class|finally|long|strictfp|volatile|const|float|native|super|while)\b").unwrap(),
        }
    }

    /// Try to lex a keyword from the given char stream. Returns None if not currently placed at a
    /// keyword. This will also lex modifiers like public / private.
    pub fn lex<'a>(&self, input: &mut CharStream<'a>) -> Option<Token<'a>> {
        let input_str = input.as_str();
        let key_match = self.re_keyword.find(input_str);
        if key_match.is_some() {
            let key_match = key_match.unwrap();
            let tok_str = input_str[0..key_match.end()].trim();
            let tok = Some(Token {
                token_type: TokenType::Key,
                val: tok_str,
            });
            input.nth(tok_str.len() - 1);
            return tok;
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::KeywordsLexer as Lexer;

    #[test]
    fn it_lexes_java_keywords() {
        test_lexing!(
            ("package com.tom.test", "package"),
            ("import com.tom.test.MyClass", "import"),
            ("class John", "class"),
            ("if (myBool)", "if"),
            ("if(", "if"),
            ("while (true)", "while"),
            ("for (;;)", "for"),
            ("public void", "public"),
            ("private int", "private"),
            ("static int", "static"),
            ("final int", "final"),
            ("synchronized void", "synchronized"),
            ("native void", "native"),
            ("strictfp void", "strictfp")
            );
    }

    #[test]
    fn it_fails_to_lex_non_java_keywords() {
        let lexer = Lexer::new();
        let mut test_str_0 = "public_asd".chars();
        let tok_0 = lexer.lex(&mut test_str_0);
        assert!(tok_0.is_none());
        assert_eq!(test_str_0.as_str(), "public_asd");
    }
}
