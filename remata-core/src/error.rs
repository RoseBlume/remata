#[derive(Debug)]
pub struct ParserError{
    pub message: String
}

impl ParserError {
    /// Creates a new ASF error with a message.
    pub fn new(msg: &str) -> Self { Self { message: msg.into() } }
}

impl From<std::io::Error> for ParserError {
    fn from(e: std::io::Error) -> Self {
        ParserError { message: e.to_string() }
    }
}