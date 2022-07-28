use estring::{Aggregate, EString, Product, SepVec, Sum};

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> estring::Result<()> {
    let res = EString::from("10+5*2+3")
        .parse::<Sum<PlusVec<Product<MulVec<f32>>>>>()?
        .agg();

    assert_eq!(res, 23.0);
    Ok(())
}
