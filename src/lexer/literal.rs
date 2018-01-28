use regex::{Regex, RegexBuilder};
use lexer::CharStream;
use lexer::token::{Token, TokenType};

/// A function for lexing literals - i.e. numbers, strings, etc. Can return an error if the literal
/// is malformed - this error will be a string message.
pub fn lex<'a>(input: &mut CharStream<'a>) -> Result<Option<Token<'a>>, &'static str> {
    let input_str = input.as_str();
    let re_digit = Regex::new("^\\d").unwrap();

    // A list of regex matching 'keyword' literals, like 'true' / 'false' / 'null'
    let keyword_literals = [
        Regex::new(r"^true\b").unwrap(),
        Regex::new(r"^false\b").unwrap(),
        Regex::new(r"^null\b").unwrap(),
        Regex::new(r"^true\b").unwrap(),
    ];
    for key in keyword_literals.iter() {
        let key_match = key.find(input_str);
        if key_match.is_some() {
            let key_match = key_match.unwrap();
            let tok_str = input_str[0..key_match.end()].trim();
            let tok = Ok(Some(Token {
                token_type: TokenType::Key,
                val: tok_str,
            }));
            input.nth(tok_str.len()-1);
            return tok;
        }
    }

    let re_digit = Regex::new("^\\d").unwrap();
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
    } else if &input_str[..1] == "'" {
        input.next();
        // Find the matching close "
        let mut c;
        let mut escape = false;
        let mut ix = 1;
        loop {
            ix += 1;
            let c_opt = input.next();
            if c_opt.is_none() {
                return Err("Unterminated character literal");
            }
            c = c_opt.unwrap();
            if !escape {
                if c == '\'' {
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
    } else if re_digit.is_match(input_str) {
        let re_number_literal = RegexBuilder::new(r"^(0x|0b)?[0-9a-f]+\.?\d*(l|f|d)?")
            .case_insensitive(true)
            .build().unwrap();
        let literal_match = re_number_literal.find(input_str).unwrap();
        input.nth(literal_match.end()-1);
        Ok(Some(Token {
            token_type: TokenType::Literal,
            val: &input_str[..literal_match.end()],
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

    #[test]
    fn it_should_lex_all_number_literals() {
        test_lexing_double_unwrap!( 
            ("1 + 2", "1"),
            ("4.0 + 4", "4.0"),
            ("4.0.0", "4.0"),
            ("40l + 50l", "40l"),
            ("0.0f + 50l", "0.0f"),
            ("0.f + 50l", "0.f"),
            ("0.F + 50.f", "0.F"),
            ("0b00001101 + 1", "0b00001101")
        );
    }

    #[test]
    fn it_should_lex_all_keyword_literals() {
        test_lexing_double_unwrap!( 
            ("true;", "true"),
            ("false; i = i + 1;", "false"),
            ("null ", "null")
        );
    }

    #[test]
    fn it_should_lex_character_literals() {
        test_lexing_double_unwrap!( 
            ("'a';", "'a'"),
            ("'\\n' == newlineChar", "'\\n'")
        );
    }

    #[test]
    fn it_should_error_with_unterminated_char_literal() {
        let mut test_str = r#"'a asd"#.chars();
        let tok_0 = lex(&mut test_str);
        assert!(
            tok_0.is_err(),
            "Unterminated char literal did not return an error"
        );
    }

}
