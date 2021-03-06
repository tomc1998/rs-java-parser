mod token;
mod error;

pub use self::token::{Token, TokenType, Point};
pub use self::error::LexErr;

use std::str::CharIndices;

#[inline]
pub fn try_comment(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (start, c) = clone.next().unwrap();
    match c {
        '/' => match clone.next() {
            // /* style comment
            Some((_, '*')) => {
                // Consume the //
                cix.next().unwrap();
                cix.next().unwrap();
                let mut end = start + 3;
                let mut found_asterisk = false;
                // Now consume until */, using found_asterisk to remember if the
                // last token was *
                while let Some((ix, c)) = cix.next() {
                    end = ix;
                    if c == '*' {
                        found_asterisk = true;
                    } else if c == '/' && found_asterisk {
                        end += 1;
                        break;
                    } else {
                        found_asterisk = false;
                    }
                }
                Ok(Some(Token::new_comment(start, end)))
            }
            // // style comment
            Some((_, '/')) => {
                // Consume the //
                cix.next().unwrap();
                cix.next().unwrap();
                let mut end = start + 3;
                // Now consume until \n or EOF
                while let Some((ix, c)) = cix.clone().next() {
                    end = ix;
                    if c == '\n' { break; }
                    cix.next().unwrap();
                }
                Ok(Some(Token::new_comment(start, end)))
            }
            _ => Ok(None),
        }
        _ => Ok(None),
    }
}

/// Checks if s starts with a punctuation char. All punc is just 1 char in this
/// lang, so just consume 1 char if a punc is found.
#[inline]
pub fn try_punc(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (ix, c) = clone.next().unwrap();
    match c {
        '.' => {
            match clone.next() {
                Some((_, '.')) => match clone.next() {
                    Some((_, '.')) => {
                        for _ in 0..3 { cix.next().unwrap(); }
                        Ok(Some(Token::new_punc(ix, ix + 1)))
                    }
                    _ => {
                        cix.next().unwrap();
                        Ok(Some(Token::new_punc(ix, ix + 1)))
                    }
                },
                _ => {
                    cix.next().unwrap();
                    Ok(Some(Token::new_punc(ix, ix + 1)))
                }
            }
        }
        ':' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | '@' | '<' | '>' => {
            cix.next().unwrap();
            Ok(Some(Token::new_punc(ix, ix + 1)))
        }
        _ => Ok(None)
    }
}

#[inline]
pub fn try_op(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (ix, c) = clone.next().unwrap();
    // Welp, this is pretty fucking messy. Fasterthanregextho.
    let (tok, consumed) = match c {
        '~' | '?' | ':' => (Token::new_op(ix, ix+1), 1),
        '=' | '/' | '*' | '^' | '%' | '!' => match clone.next() {
            Some((_, '=')) => (Token::new_op(ix, ix+2), 2),
            _ => (Token::new_op(ix, ix+1), 1)
        },
        '+' => match clone.next() {
            Some((_, '=')) | Some((_, '+')) => (Token::new_op(ix, ix+2), 2),
            _ => (Token::new_op(ix, ix+1), 1),
        },
        '-' => match clone.next() {
            Some((_, '=')) | Some((_, '-')) | Some((_, '>')) => (Token::new_op(ix, ix+2), 2),
            _ => (Token::new_op(ix, ix+1), 1),
        },
        '>' => match clone.next() {
            Some((_, '=')) => (Token::new_op(ix, ix+2), 2),
            Some((_, '>')) => match clone.next() {
                Some((_, '>')) => match clone.next() {
                    Some((_, '=')) => (Token::new_op(ix, ix+4), 4),
                    _ => (Token::new_op(ix, ix+3), 3),
                }
                Some((_, '=')) => (Token::new_op(ix, ix+3), 3),
                _ => (Token::new_op(ix, ix+2), 2),
            },
            _ => (Token::new_op(ix, ix+1), 1)
        }
        '<' => {
            match clone.next() {
                Some((_, '=')) => (Token::new_op(ix, ix+2), 2),
                Some((_, '<')) => match clone.next() {
                    Some((_, '=')) => (Token::new_op(ix, ix+3), 3),
                    _ => (Token::new_op(ix, ix+2), 2),
                }
                _ => (Token::new_op(ix, ix+1), 1),
            }
        }
        '&' => {
            match clone.next() {
                Some((_, '&')) | Some((_, '=')) => (Token::new_op(ix, ix+2), 2),
                _ => (Token::new_op(ix, ix+1), 1),
            }
        }
        '|' => {
            match clone.next() {
                Some((_, '|')) | Some((_, '=')) => (Token::new_op(ix, ix+2), 2),
                _ => (Token::new_op(ix, ix+1), 1),
            }
        }
        _ => return Ok(None)
    };
    for _ in 0..consumed { cix.next(); }
    Ok(Some(tok))
}

#[inline]
pub fn try_key(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let (start, _) = cix.clone().next().unwrap();
    const KEYS : [&str; 50] =
        ["abstract", "continue", "for", "new", "switch", "assert", "default",
         "goto", "package", "synchronized", "boolean", "do", "if", "private", "this",
         "break", "double", "implements", "protected", "throw", "byte", "else",
         "import", "public", "throws", "case", "enum", "instanceof", "return",
         "transient", "catch", "extends", "int", "short", "try", "char", "final",
         "interface", "static", "void", "class", "finally", "long", "strictfp",
         "volatile", "const", "float", "native", "super", "while"];
    for k in KEYS.iter() {
        if cix.as_str().starts_with(k) {
            return match cix.clone().skip(k.len()).next() {
                None => {
                    for _ in 0..k.len() { cix.next(); } // Consume
                    Ok(Some(Token::new_key(start, start + k.len())))
                }
                Some((_, c)) if !c.is_alphanumeric() => {
                    for _ in 0..k.len() { cix.next(); } // Consume
                    Ok(Some(Token::new_key(start, start + k.len())))
                }
                _ => {
                    Ok(None)
                }
            }
        }
    }
    Ok(None)
}

#[inline]
pub fn try_char_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    if cix.clone().next().unwrap().1 == '\'' {
        let (start, _) = cix.next().unwrap();
        // Keep consuming until we hit another unescaped "
        let mut escaped = false;
        let mut end = None;
        while let Some((ix, c)) = cix.next() {
            if escaped {
                escaped = false;
                continue;
            } else if c == '\\' {
                escaped = true;
                continue;
            } else if c == '\'' {
                end = Some(ix + 1);
                break;
            }
        }
        match end {
            None => Err(LexErr::Raw("Unexpected EOF in char literal".to_owned())),
            Some(end) => Ok(Some(Token::new_string_lit(start, end)))
        }
    } else { Ok(None) }
}

pub fn try_string_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    if cix.clone().next().unwrap().1 == '"' {
        let (start, _) = cix.next().unwrap();
        // Keep consuming until we hit another unescaped "
        let mut escaped = false;
        let mut end = None;
        while let Some((ix, c)) = cix.next() {
            if escaped {
                escaped = false;
                continue;
            } else if c == '\\' {
                escaped = true;
                continue;
            } else if c == '"' {
                end = Some(ix + 1);
                break;
            }
        }
        match end {
            None => Err(LexErr::Raw("Unexpected EOF in string literal".to_owned())),
            Some(end) => Ok(Some(Token::new_string_lit(start, end)))
        }
    } else { Ok(None) }
}

#[inline]
pub fn try_num_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (start, first) = clone.next().unwrap();
    if first.is_digit(10) || first == '-' {
        // Consume until we hit a non-digit
        let mut num_consumed = 1;
        let mut consumed_decimal_point = false;
        let mut end = start;
        while let Some((ix, c)) = clone.next() {
            if c.is_alphabetic() {
                return Err(LexErr::Raw("Identifier cannot start with a number".to_owned()));
            } else if c == '.' && consumed_decimal_point {
                return Err(LexErr::Raw("Error: num literal contains more than 1 decimal place".to_owned()));
            } else if !c.is_digit(10) && c != '.' {
                break;
            } else if c == '.' {
                consumed_decimal_point = true;
            }
            num_consumed += 1;
            end = ix;
        }
        for _ in 0..num_consumed { cix.next(); } // Advance the iterator
        if consumed_decimal_point {
            Ok(Some(Token::new_float_lit(start, end+1)))
        } else {
            Ok(Some(Token::new_int_lit(start, end+1)))
        }
    } else { Ok(None) }
}

#[inline]
pub fn try_null_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let start = cix.clone().next().unwrap().0;
    if cix.as_str().starts_with("null") {
        return match cix.clone().skip(4).next() {
            Some((_, c)) if !c.is_alphanumeric() => {
                for _ in 0..4 { cix.next(); } // Consume
                Ok(Some(Token::new_null_lit(start, start + 4)))
            }
            None => {
                for _ in 0..4 { cix.next(); } // Consume
                Ok(Some(Token::new_null_lit(start, start + 4)))
            }
            _ => Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[inline]
pub fn try_bool_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let start = cix.clone().next().unwrap().0;
    let as_str = cix.as_str();
    if let Some((tok, to_consume)) = if as_str.starts_with("true") {
        let len = "true".len();
        Some((Token::new_bool_lit(start, start + len), len))
    } else if as_str.starts_with("false") {
        let len = "false".len();
        Some((Token::new_bool_lit(start, start + len), len))
    } else { None } {
        for _ in 0..to_consume { cix.next(); }
        Ok(Some(tok))
    } else {
        Ok(None)
    }
}

#[inline]
pub fn try_ident(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (start, first) = clone.next().unwrap();
    if first.is_alphabetic() {
        // Consume until we hit a non-alphanumeric
        let mut num_consumed = 1;
        let mut end = start + 1;
        while let Some((ix, c)) = clone.next() {
            if !(c.is_alphanumeric() || c == '_') { break; }
            num_consumed += 1;
            end = ix + 1;
        }
        for _ in 0..num_consumed { cix.next(); } // Advance the iterator
        Ok(Some(Token::new_ident(start, end)))

    } else { Ok(None) }
}

pub fn lex_token(cix: &mut CharIndices) -> Result<Token, LexErr> {
    if let Some(tok) = try_comment(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_op(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_punc(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_num_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_bool_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_null_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_key(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_char_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_string_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_ident(cix)? {
        Ok(tok)
    } else {
        return Err(LexErr::Raw("Unknown token".to_owned()));
    }
}

pub fn lex(src: &str, file: &str) -> Result<Vec<Token>, LexErr> {
    if src.is_empty() {
        return Err(LexErr::Raw("File is empty.".to_owned()));
    }

    let mut tokens = Vec::new();
    let mut char_ix = src.char_indices();
    let mut line_num = 0;

    while !char_ix.as_str().is_empty() {
        // Check if this is a newline, and increment line_num
        if char_ix.clone().next().unwrap().1 == '\n' {
            char_ix.next();
            line_num += 1;
            continue;
        } else if char_ix.clone().next().unwrap().1.is_whitespace() {
            // Just consume whitespace
            char_ix.next();
            continue;
        }

        // Try lex a token
        match lex_token(&mut char_ix) {
            Ok(tok) => tokens.push(tok),
            Err(e) => return Err(e.into_point(file.to_string(), line_num)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use std;

    #[test]
    fn test_lex() {
        let src = std::str::from_utf8(include_bytes!("../../res/test-src/com/tom/Main.java"))
            .unwrap();
        let res = super::lex(src, "com/tom/Main.java").unwrap();
        assert!(res.len() > 0);
    }

    #[test]
    fn test_lex_op() {
        let num_ops =
            [">>>=", ">>=", "<<=", ">>>", "%=", "^=", "=", "&=", "/=",
            "*=", "-=", "+=", ">>", "<<", "--", "++", "||", "&&", "!=",
            "<=", ">=", "==", "->", "%", "^", "|", "&", "/", "*", "-", "+",
            ":", "?", "~", "!", "<", "=", ">"].len();
        let ops = ">>>=\n>>=\n<<=\n>>>\n%=\n^=\n=\n&=\n/=\n\n*=\n-=\n+=\n>>\n<<".to_owned()
            + "\n--\n++\n||\n&&\n!=\n\n<=\n>=\n==\n->\n%\n^\n|\n&\n/\n*\n-\n+"
            + "\n\n:\n?\n~\n!\n<\n=\n>";

        let tokens = super::lex(&ops, "").unwrap();
        assert_eq!(tokens.len(), num_ops);
        assert!(tokens.iter().all(|t| t.token_type == super::TokenType::Op));
    }
}

#[cfg(feature = "bench")]
mod benches {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn test_lex(b: &mut Bencher) {
        let java_code = r#"
        package com.tom.test;
        public class Main {
            public static void main(String[] args) {
                // Hello, this is a // style comment
                /* Hello, this is a /* style comment */
                float a = 3.f;
                float b = .2f;
                float c = a + b;
                System.out.println("Hello, world!");
                System.out.println("3 + 0.2 = " + c);
            }
        }
        "#;
        b.iter(|| test::black_box(super::lex(java_code, "")));
    }
}
