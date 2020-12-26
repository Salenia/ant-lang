#![allow(dead_code)]
extern crate common;
use common::error::{PResult, ParserError};
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
    type Output = PResult<Vec<Token>>;

    fn build(&mut self, input: Self::Input) -> Self::Output {
        Err(ParserError::Syntax("".to_string()))
    }
}
