#[derive(Debug, Clone, PartialEq)]
pub enum Symbols {
    Not,       // !
    Assign,    // =
    Comma,     // ,
    Dot,       // .
    SemiColon, // ;
    Colon,     // :
    LT,        // <
    RT,        // >
    LParan,    // (
    RParan,    // )
    LBrace,    // {
    RBrace,    // }
    LBraket,   // [
    RBraket,   // ]
    EOF,
}
