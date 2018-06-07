//! Module containing functions to lex / parse whole source folders

use std;
use std::fs;
use std::path::{Path, PathBuf};
use std::convert::AsRef;
use lexer::{Token, TokenType, lex, LexErr};

/// A list of sources, which can be lexed to produce a LexedSourceFolder (maintaining a borrow on
/// this struct)
pub struct SourceFolder {
    /// A list of files (first) paired with file path names (second)
    pub source_lists: Vec<(String, PathBuf)>,
}

/// Contains a list of lexed tokens mapped to file names, and a list of child directories.
pub struct LexedSourceFolder<'a> {
    /// A list of token lists paired with file paths
    pub token_lists: Vec<(Vec<Token>, &'a Path)>,
}

impl SourceFolder {
    /// Read all .java files recursively in a folder. This will never return if the directory
    /// structure hard loops.
    ///
    /// **Sym links are ignored.**
    pub fn read<P: AsRef<Path>>(path: P) -> Result<SourceFolder, std::io::Error> {
        let mut source_folder = SourceFolder { source_lists: Vec::new() };

        let mut entry_list = Vec::new();
        entry_list.push(try!(fs::read_dir(path)));

        while !entry_list.is_empty() {
            let entry = entry_list.pop().unwrap();
            for f in entry {
                let f = try!(f);
                let ft = try!(f.file_type());
                let path = f.path();
                if ft.is_dir() {
                    entry_list.push(try!(fs::read_dir(path)));
                } else if ft.is_file() && path.as_path().extension().unwrap() == "java" {
                    use std::io::Read;
                    let mut f = try!(fs::File::open(path.as_path()));
                    let mut buf = String::new();
                    try!(f.read_to_string(&mut buf));
                    source_folder.source_lists.push((buf, path));
                }
            }
        }

        return Ok(source_folder);
    }

    pub fn lex<'a>(&'a self) -> Result<LexedSourceFolder<'a>, LexErr> {
        let mut lexed = LexedSourceFolder { token_lists: Vec::new() };

        for &(ref s, ref p) in &self.source_lists {
            lexed.token_lists.push((lex(&s, p.to_str().unwrap())?, p.as_path()));
        }

        return Ok(lexed);
    }
}

impl<'a> LexedSourceFolder<'a> {
    /// Strip the comments from the source. This will be necessary for most parsing.
    pub fn strip_comments(&mut self) {
        for &mut (ref mut token_list, _) in &mut self.token_lists {
            let mut ix = 0;
            loop {
                if ix >= token_list.len() {
                    break;
                }
                let tok = token_list[ix];
                if tok.token_type == TokenType::Comment {
                    token_list.remove(ix);
                } else {
                    ix += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SourceFolder;
    use lexer::TokenType;

    #[test]
    fn test_source_is_read() {
        let source_folder =
            SourceFolder::read("res/test-src").expect("Source folder failed to read");
        for &(ref s, ref p) in &source_folder.source_lists {
            assert!(!s.is_empty(), "Java source file empty");
            assert!(p.as_path().exists(), "Path to file doesn't exist");
            assert_eq!(
                p.as_path().extension().unwrap(),
                "java",
                "Read non-java file"
            );
        }
    }

    #[test]
    fn test_source_is_all_lexed() {
        let source_folder =
            SourceFolder::read("res/test-src").expect("Source folder failed to read");
        let lexed = source_folder.lex().expect("Lex failed");
        for &(ref tokens, ref p) in &lexed.token_lists {
            if p.file_name().unwrap() == "Main.java" {
                assert!(tokens.iter().any(|t| {
                    t.token_type == TokenType::Ident && t.val == "Main"
                }));
            }
            if p.file_name().unwrap() == "Person.java" {
                assert!(tokens.iter().any(|t| {
                    t.token_type == TokenType::Ident && t.val == "Person"
                }));
            }
            assert!(!tokens.is_empty(), "No tokens in lexed file");
            assert!(p.exists(), "Path to lexed file doesn't exist");
        }
    }

    #[test]
    fn test_strip_comments() {
        let source_folder =
            SourceFolder::read("res/test-src").expect("Source folder failed to read");
        let mut lexed = source_folder.lex();
        lexed.strip_comments();
        for &(ref token_list, _) in &lexed.token_lists {
            for t in token_list {
                assert_ne!(t.token_type, TokenType::Comment);
            }
        }
    }
}
