use regex::Regex;
use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// Try to lex operator chars from the given char stream.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Option<Token<'a>> {
    let input_str = input.as_str();
    // Believe it or not, this is a regex for any java operator. Copied from the docs, sorted by
    // length. Obviously incredibly messy, but if you really wanted to unpack this replace | with
    // |\r - it's just a massive alternation.
    let re_punc = Regex::new(
r"^(>>>=|>>=|<<=|>>>|%=|\^=|\|=|&=|/=|\*=|-=|\+=|>>|<<|--|\+\+|\|\||&&|!=|<=|>=|==|->|%|\^|\||&|/|\*|-|\+|:|\?|~|!|<|=|>)").unwrap();
    let punc_match = re_punc.find(input_str);
    if punc_match.is_some() {
        let punc_match = punc_match.unwrap();
        input.nth(punc_match.end() - 1);
        Some(Token {
            token_type: TokenType::Punc,
            val: &input_str[..punc_match.end()],
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn it_lexes_valid_operators() {
        test_lexing!( 
            ("=asd", "="),
            (">asd", ">"),
            ("<asd", "<"),
            ("!asd", "!"),
            ("~asd", "~"),
            ("? asd", "?"),
            (": asd", ":"),
            ("-> asd", "->"),
            ("== asd", "=="),
            (">= asd", ">="),
            ("<= asd", "<="),
            ("!= asd", "!="),
            ("&& asd", "&&"),
            ("|| asd", "||"),
            ("++ asd", "++"),
            ("--asd", "--"),
            ("+asd", "+"),
            ("-asd", "-"),
            ("*asd", "*"),
            ("/asd", "/"),
            ("&asd", "&"),
            ("|asd", "|"),
            ("^asd", "^"),
            ("%asd", "%"),
            ("<<asd", "<<"),
            (">>asd", ">>"),
            (">>>2.0", ">>>"),
            ("+=2.0", "+="),
            ("-=2.0", "-="),
            ("*=2.0", "*="),
            ("/=2.0", "/="),
            ("&=2.0", "&="),
            ("|=2.0", "|="),
            ("^=2.0", "^="),
            ("%=2.0", "%="),
            ("<<=2.0", "<<="),
            (">>=2.0", ">>="),
            (">>>=asd", ">>>=")
        );
    }

    #[test]
    fn it_fails_to_lex_non_operator_chars() {
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
