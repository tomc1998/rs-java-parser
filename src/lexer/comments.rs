use lexer::CharStream;
use lexer::token::{Token, TokenType};

pub struct CommentsLexer {}

impl CommentsLexer {
    pub fn new() -> CommentsLexer {
        CommentsLexer {}
    }

    /** Performs lexing for comments. Will return an Err if the comment is unterminated (i.e. /*
     * with no matching */ ).
     */
    pub fn lex<'a>(&self, input: &mut CharStream<'a>) -> Result<Option<Token<'a>>, &'static str> {
        let input_str : &str = input.as_str();
        if input_str.starts_with("/*") {
            // Find matching */. We don't need to care about nesting, as javac doesn't support
            // that.
            let closing_ix = input_str.find("*/");
            if closing_ix.is_none() {
                Err("Unclosed comment")
            }
            else {
                let closing_ix = closing_ix.unwrap();
                input.nth(closing_ix+1);
                Ok(Some(Token {
                    token_type: TokenType::Comment,
                    val: &input_str[..closing_ix+2]
                }))
            }
        }
        else if input_str.starts_with("//") {
            let line = input_str.lines().next().unwrap();
            input.nth(line.len()-1);
            Ok(Some(Token {
                token_type: TokenType::Comment,
                val: line,
            }))
        }
        else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommentsLexer as Lexer;

    #[test]
    fn it_should_lex_valid_comments() {
        test_lexing_double_unwrap!(
            ("// asdlkjasdlkj", "// asdlkjasdlkj"),
            ("/* Hello, this is a comment */ asd", "/* Hello, this is a comment */")
            );
    }

    #[test]
    fn it_should_fail_to_lex_non_comments() {
        let lexer = Lexer::new();
        assert!(lexer.lex(&mut "Hello".chars()).unwrap().is_none());
        assert!(lexer.lex(&mut "/ *aasdad".chars()).unwrap().is_none());
        assert!(lexer.lex(&mut "/ /asdlkjasd".chars()).unwrap().is_none());
    }

    #[test]
    fn it_should_error_with_unterminated_comment() {
        let lexer = Lexer::new();
        assert!(lexer.lex(&mut "/* alskdjasd".chars()).is_err());
    }
}
