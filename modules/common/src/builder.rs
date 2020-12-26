// One impl per struct
pub trait Builder {
    type Input;
    type Output;
    fn build(&mut self, input: Self::Input) -> Self::Output;
}
