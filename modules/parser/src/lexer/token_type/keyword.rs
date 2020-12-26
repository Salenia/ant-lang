#[derive(Debug, Clone, PartialEq, Copy)]
pub enum KeywordType {
    Let,
    While,
    For,
    If,
    Else,
    Function,
    Null,
    True,
    False,
}
impl KeywordType {
    pub fn from(input: &str) -> Option<Self> {
        match input {
            "let" => Some(Self::Let),
            "while" => Some(Self::While),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "function" => Some(Self::Function),
            "null" => Some(Self::Null),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            _ => None,
        }
    }
}
