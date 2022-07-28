use std::marker::PhantomData;

use crate::{Aggregatable, Aggregate, EString, ParseFragment};

#[derive(Debug, PartialEq, Eq)]
struct Sum<R, T>(T, PhantomData<R>);

impl<R, T> Sum<R, T> {
    fn new(inner: T) -> Self {
        Self(inner, PhantomData::default())
    }
}

impl<R, T> ParseFragment for Sum<R, T>
where
    T: ParseFragment,
{
    fn parse_frag(es: EString) -> crate::Result<Self> {
        T::parse_frag(es).map(Self::new)
    }
}

impl<R, T> Aggregate for Sum<R, T>
where
    R: std::iter::Sum,
    T: Aggregatable<Item = R>,
{
    type Target = R;

    fn agg(self) -> Self::Target {
        self.0.items().into_iter().sum()
    }
}

impl<R, T> Aggregatable for Sum<R, T>
where
    R: std::iter::Sum,
    T: Aggregatable<Item = R>,
{
    type Item = R;

    fn items(self) -> Vec<Self::Item> {
        vec![self.agg()]
    }
}

#[cfg(test)]
mod tests {
    use crate::SepVec;

    use super::*;

    type CommaVec<T> = SepVec<T, ','>;
    type PlusVec<T> = SepVec<T, '+'>;

    #[test]
    fn should_parse_vec() {
        let es = EString::from("1,2,3");
        match es.parse::<Sum<i32, CommaVec<i32>>>() {
            Ok(res) => assert_eq!(res, Sum::new(CommaVec::from(vec![1, 2, 3]))),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_aggregate_vector() {
        let es = EString::from("1,2,3");
        let expr = es.parse::<Sum<i32, CommaVec<i32>>>().unwrap();
        assert_eq!(expr.agg(), 6);
    }

    #[test]
    fn should_aggregate_vector_with_inner_vector() {
        let es = EString::from("1+2,2,3");
        let expr = es.parse::<Sum<i32, CommaVec<PlusVec<i32>>>>().unwrap();
        assert_eq!(expr.agg(), 8);
    }

    #[test]
    fn should_aggregate_vector_with_inner_aggregation() {
        let es = EString::from("1+2,2,3");
        let expr = es
            .parse::<Sum<_, CommaVec<Sum<_, PlusVec<i32>>>>>()
            .unwrap();
        assert_eq!(expr.agg(), 8);
    }
}
