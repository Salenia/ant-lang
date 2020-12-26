use super::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: usize,
    pub column: usize,

    pub token_type: TokenType,
}

impl Token {
    pub fn new(line: usize, column: usize, token_type: TokenType) -> Self {
        Self {
            line,
            column,
            token_type,
        }
    }
}
impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "Token: {:?} At Ln {} Col {}",
            self.token_type, self.line, self.column
        )
    }
}
