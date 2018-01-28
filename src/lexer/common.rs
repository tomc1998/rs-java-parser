use lexer::CharStream;

pub fn consume_whitespace<'a>(input: &mut CharStream<'a>) {
    let whitespace_chars = [" ", "\t", "\r", "\n"];
    while input.as_str().len() > 0 && whitespace_chars.contains(&&input.as_str()[..1]) {
        input.next();
    }
}
