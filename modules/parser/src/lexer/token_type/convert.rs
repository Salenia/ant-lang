pub trait Convert<TIn, TOut> {
    fn convert(input: TIn) -> TOut;
}
