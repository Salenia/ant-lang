#[derive(Debug, Clone, PartialEq, Hash, Copy)]
pub enum PremitiveType {
    Int,
    Float,
    Bool,
    Str,
    Char,
    Tuple,
    Array,
}
