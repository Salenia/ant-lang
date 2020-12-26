#![allow(dead_code)]
mod token;
mod token_type;

pub use token::Token;
pub use token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Lexer {
    content: String,
    index: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            content: String::from(input),
            index: 0,
        }
    }
}
