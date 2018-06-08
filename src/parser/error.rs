use lexer::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParseErr {
    /// Just an err message
    Raw(String),
    /// Err message with token for location
    Point(String, Token),
}

impl ParseErr {
    /// Print this error
    pub fn print_formatted(&self, filename: &str, src: &str) {
        match *self {
            ParseErr::Raw(ref s) => println!("{} - {}", filename, s),
            ParseErr::Point(ref s, ref t) => {
                // find the line num
                let mut line_num = 1;
                for (ix, c) in src.char_indices() {
                    if ix == t.start.0 { break; }
                    if c == '\n' {
                        line_num += 1;
                    }
                }

                println!("{} - {}:{}", s, filename, line_num);
            }
        }
    }
}

