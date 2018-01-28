use regex::Regex;
use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// Try to lex an identifier from the given char stream. This WILL lex keywords too, so make sure
/// to run the keyword lexer before this to consume any keywords before running the identifier
/// lexer.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Option<Token<'a>> {
    let input_str = input.as_str();
    let re_starts_with_char = Regex::new("^[A-Za-z]").unwrap();
    if re_starts_with_char.is_match(input_str) {
        let re_first_non_ident_char = Regex::new("[^A-Za-z0-9]").unwrap();
        let res = re_first_non_ident_char.find(input_str);
        if res.is_none() {
            input.count(); // Consume the whole iter
            Some(Token {
                token_type: TokenType::Ident,
                val: input_str
            })
        }
        else {
            let res = res.unwrap().start();
            input.nth(res-1); // Consume iter up to the end of the ident
            Some(Token {
                token_type: TokenType::Ident,
                val: &input_str[0..res],
            })
        }
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn it_lexes_valid_identifiers() {
        let mut test_str_0 = "myVar.callFunc();".chars();
        let mut test_str_1 = "myVar = \"hello\";".chars();
        let mut test_str_2 = "com.tom.MyClass".chars();
        let tok_0 = lex(&mut test_str_0).expect("Identifier not lexed");
        let tok_1 = lex(&mut test_str_1).expect("Identifier not lexed");
        let tok_2 = lex(&mut test_str_2).expect("Identifier not lexed");
        assert_eq!(tok_0.val, "myVar");
        assert_eq!(tok_1.val, "myVar");
        assert_eq!(tok_2.val, "com");
        assert_eq!(test_str_0.as_str(), ".callFunc();");
        assert_eq!(test_str_1.as_str(), " = \"hello\";");
        assert_eq!(test_str_2.as_str(), ".tom.MyClass");
    }

    #[test]
    fn it_fails_to_lex_invalid_identifiers() {
        let mut test_str_0 = "123.0".chars();
        let mut test_str_1 = "1myInvalidVar".chars();
        let mut test_str_2 = ".callFunc()".chars();
        let tok_0 = lex(&mut test_str_0);
        let tok_1 = lex(&mut test_str_1);
        let tok_2 = lex(&mut test_str_2);
        assert!(tok_0.is_none());
        assert!(tok_1.is_none());
        assert!(tok_2.is_none());
        assert_eq!(test_str_0.as_str(), "123.0");
        assert_eq!(test_str_1.as_str(), "1myInvalidVar");
        assert_eq!(test_str_2.as_str(), ".callFunc()");
    }
}
