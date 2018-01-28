use regex::Regex;
use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// Try to lex punctuation character from the given char stream.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Option<Token<'a>> {
    let input_str = input.as_str();
    let re_punc = Regex::new(r"^[\.\(\)\[\]{};@]").unwrap();
    if re_punc.is_match(input_str) {
        input.next();
        Some(Token {
            token_type: TokenType::Punc,
            val: &input_str[0..1],
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn it_lexes_valid_punctuation() {
        let test_punc_chars_str = "{}[].;@";
        let mut test_punc_chars = test_punc_chars_str.chars();
        for _ in 0..test_punc_chars_str.len() {
            let punc_str = &test_punc_chars.as_str()[0..1];
            let tok = lex(&mut test_punc_chars);
            assert_eq!(tok.unwrap().val, punc_str);
        }
        assert_eq!(test_punc_chars.as_str(), "");
    }

    #[test]
    fn it_fails_to_lex_non_punc_chars() {
        let mut test_str_0 = "123.0".chars();
        let mut test_str_1 = "1myInvalidVar".chars();
        let mut test_str_2 = "myObj.callFunc()".chars();
        let tok_0 = lex(&mut test_str_0);
        let tok_1 = lex(&mut test_str_1);
        let tok_2 = lex(&mut test_str_2);
        assert!(tok_0.is_none());
        assert!(tok_1.is_none());
        assert!(tok_2.is_none());
        assert_eq!(test_str_0.as_str(), "123.0");
        assert_eq!(test_str_1.as_str(), "1myInvalidVar");
        assert_eq!(test_str_2.as_str(), "myObj.callFunc()");
    }
}
