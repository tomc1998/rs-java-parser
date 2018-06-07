#[derive(Eq, PartialEq, Debug, Clone)]
pub enum LexErr {
    /// Just an err message
    Raw(String),
    /// Msg, file, line num
    Point(String, String, usize),
}

impl LexErr {
    pub fn into_point(self, file: String, line_num: usize) -> Self {
        match self {
            LexErr::Raw(s) => LexErr::Point(s, file, line_num),
            _ => panic!("Trying to convert a LexErr::Point into a LexErr::Point!"),
        }
    }

    /// Print this error
    pub fn print_formatted(&self) -> String {
        match *self {
            LexErr::Raw(ref s) => format!("Error: {}", s),
            LexErr::Point(ref s, ref f, ref l) => format!("Error: {} - {}:{}", s, f, l),
        }
    }
}

