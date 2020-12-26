use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
    Unexpected(String), // value of token || token.to_string()
    Syntax(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Syntax(val) => write!(f, "SyntaxError: invalid syntax for : {}", val),
            Self::Unexpected(val) => write!(f, "Unexpected: unexpected token: {}", val),
        }
    }
}
pub type PResult<T> = Result<T, ParserError>;
