use estring::structs::SepVec;
use estring::EString;

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> Result<(), estring::ParseError> {
    let res = EString::from("10+5*2+3")
        .parse::<PlusVec<MulVec<f32>>>()?
        .iter()
        .map(|m| m.iter().product::<f32>())
        .sum::<f32>();

    assert_eq!(res, 23.0);
    Ok(())
}
