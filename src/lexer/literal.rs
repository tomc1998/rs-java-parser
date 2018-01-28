use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// A function for lexing literals - i.e. numbers, strings, etc. Can return an error if the literal
/// is malformed - this error will be a string message.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Result<Option<Token<'a>>, &'static str> {
    let input_str = input.as_str();
    if &input_str[..1] == "\"" {
        input.next();
        // Find the matching close "
        let mut c;
        let mut escape = false;
        let mut ix = 1;
        loop {
            ix += 1;
            let c_opt = input.next();
            if c_opt.is_none() {
                return Err("Unterminated string literal");
            }
            c = c_opt.unwrap();
            if !escape {
                if c == '"' {
                    break;
                } else if c == '\\' {
                    escape = true;
                }
            } else {
                escape = false;
            }
        }
        Ok(Some(Token {
            token_type: TokenType::Literal,
            val: &input_str[..ix],
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn it_should_lex_valid_string_literal() {
        let mut test_str = r#""Hello, this is a string literal" and this is not"#.chars();
        let tok_0 = lex(&mut test_str).unwrap().expect(
            "String literal not lexed",
        );
        assert_eq!(tok_0.val, r#""Hello, this is a string literal""#);
        assert_eq!(test_str.as_str(), " and this is not");
    }

    #[test]
    fn it_should_error_with_unterminated_string_literal() {
        let mut test_str = r#""Hello, this is an unterminated string literal "#.chars();
        let tok_0 = lex(&mut test_str);
        assert!(
            tok_0.is_err(),
            "Unterminated string literal did not return an error"
        );
    }

    #[test]
    fn it_should_return_none_with_no_literal() {
        let mut test_str = "ident.callFunc()".chars();
        let tok_0 = lex(&mut test_str).unwrap();
        assert!(tok_0.is_none());
    }
}
