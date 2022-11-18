use polars::prelude::DataFrame;

pub trait Transformer: Sized {
    type Err;

    fn transform(self) -> Result<DataFrame, Self::Err>;
}
