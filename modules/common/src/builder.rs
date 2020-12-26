pub trait Builder<TIn, TOut> {
    fn build(input: TIn) -> TOut;
}
