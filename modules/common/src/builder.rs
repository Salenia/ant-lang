// One impl per struct
pub trait Builder {
    type Input;
    type Output;
    fn build(input: Self::Input) -> Self::Output;
}
