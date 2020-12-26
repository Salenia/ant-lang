#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ConditionType {
    NotEqual,
    Equal,
    Grater,
    Smaller,
    GraterEqual,
    SmallerEqual,
    BAnd,
    BOr,
    And,
    Or,
}

impl ConditionType {
    pub fn priority(&self) -> i8 {
        match self {
            Self::BAnd | Self::BOr | Self::And | Self::Or => -2,
            _ => -1,
        }
    }
}

impl super::Convert<&str, Option<Self>> for ConditionType {
    fn convert(input: &str) -> Option<Self> {
        match input {
            "==" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            ">" => Some(Self::Grater),
            "<" => Some(Self::Smaller),
            ">=" => Some(Self::GraterEqual),
            "<=" => Some(Self::SmallerEqual),
            "&&" => Some(Self::And),
            "&" => Some(Self::BAnd),
            "||" => Some(Self::Or),
            "|" => Some(Self::BOr),
            _ => None,
        }
    }
}
