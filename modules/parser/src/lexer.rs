#![allow(dead_code)]
extern crate common;
use common::Builder;

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
            content: input,
            index: 0,
        }
    }
}
impl Builder for Lexer {
    type Input = String;
    type Output = Result<Vec<Token>, String>;

    fn build(input: String) -> Result<Vec<Token>, String> {
        Err("".to_string())
    }
}
