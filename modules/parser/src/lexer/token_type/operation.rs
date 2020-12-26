#[derive(Debug, Clone, PartialEq, Copy)]
pub enum OperationType {
    ModuleEqual,
    PlusEqual,
    MinusEqual,
    DevideEqual,
    MulEqual,
    Mul,
    Devide,
    Minus,
    Plus,
    Power,
    Module,
}
impl OperationType {
    pub fn is_to_assign(&self) -> bool {
        match self {
            Self::PlusEqual
            | Self::MinusEqual
            | Self::DevideEqual
            | Self::MulEqual
            | Self::ModuleEqual => true,
            _ => false,
        }
    }
    pub fn get_assign_op(&self) -> Option<Self> {
        match self {
            Self::MulEqual => Some(Self::Mul),
            Self::DevideEqual => Some(Self::Devide),
            Self::MinusEqual => Some(Self::Minus),
            Self::PlusEqual => Some(Self::Plus),
            Self::ModuleEqual => Some(Self::Module),
            _ => None,
        }
    }

    pub fn get_priority(&self) -> Option<i8> {
        match self {
            Self::Devide | Self::Mul | Self::Module => Some(6),
            Self::Minus | Self::Plus => Some(5),
            Self::Power => Some(7),
            _ => None,
        }
    }
}
