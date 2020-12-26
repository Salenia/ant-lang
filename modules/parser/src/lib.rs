extern crate common;

mod lexer;

use lexer::Token;

pub struct Parser<'a> {
    content: &'a [Token],
    index: usize,
}

// impl<'a> common::Builder for Parser<'a> {
//     type Input = &'a [Token];
//     type Output = Result<..., String>;
// }
#[cfg(test)]
mod tests {
    use super::*;
}
