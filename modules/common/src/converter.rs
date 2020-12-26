//multiple impls for struct
pub trait Converter<TIn, TOut> {
    fn convert(input: TIn) -> TOut;
}
