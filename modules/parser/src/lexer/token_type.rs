extern crate common;
use common::Converter;

mod condition;
mod keyword;
mod operation;
mod premitive;
mod symbols;

use condition::ConditionType;
use keyword::KeywordType;
use operation::OperationType;
use premitive::PremitiveType;
use symbols::Symbols;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(KeywordType),  // let | while | each | in
    Ident(String),         // variable
    StringLiteral(String), // "hello world"
    IntLiteral(String),    // 2020
    FloatLiteral(String),  // 3.141569
    Premitive(PremitiveType),
    Type(String),

    Condition(ConditionType),
    Operation(OperationType),

    Symbol(Symbols),
}
