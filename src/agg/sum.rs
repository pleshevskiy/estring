use std::marker::PhantomData;

use crate::{Aggregate, EString, ParseFragment};

#[derive(Debug, PartialEq, Eq)]
struct Sum<R, T>(T, PhantomData<R>)
where
    R: Copy + std::iter::Sum,
    T: ParseFragment + std::ops::Deref<Target = Vec<R>>;

impl<R, T> Sum<R, T>
where
    R: Copy + std::iter::Sum,
    T: ParseFragment + std::ops::Deref<Target = Vec<R>>,
{
    fn new(inner: T) -> Self {
        Self(inner, PhantomData::default())
    }
}

impl<R, T> ParseFragment for Sum<R, T>
where
    R: Copy + std::iter::Sum,
    T: ParseFragment + std::ops::Deref<Target = Vec<R>>,
{
    fn parse_frag(es: EString) -> crate::Result<Self> {
        T::parse_frag(es).map(Self::new)
    }
}

impl<R, T> Aggregate for Sum<R, T>
where
    R: Copy + std::iter::Sum,
    T: ParseFragment + std::ops::Deref<Target = Vec<R>>,
{
    type Target = R;

    fn agg(&self) -> Self::Target {
        self.0.iter().copied().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::SepVec;

    use super::*;

    #[test]
    fn should_parse_vec() {
        let es = EString::from("1,2,3");
        match es.parse::<Sum<i32, SepVec<i32, ','>>>() {
            Ok(res) => assert_eq!(res, Sum::new(SepVec::from(vec![1, 2, 3]))),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_aggregate_vector() {
        let es = EString::from("1,2,3");
        let expr = es.parse::<Sum<i32, SepVec<i32, ','>>>().unwrap();
        assert_eq!(expr.agg(), 6);
    }

    #[test]
    fn should_aggregate_vector_with_inner_aggregation() {
        let es = EString::from("1+2,2,3");
        let expr = es
            .parse::<Sum<_, SepVec<Sum<_, SepVec<i32, '+'>>, ','>>>()
            .unwrap();
        assert_eq!(expr.agg(), 6);
    }
}
