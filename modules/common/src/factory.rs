pub trait Factory<TIn> {
    fn create(input: TIn) -> Self;
}
