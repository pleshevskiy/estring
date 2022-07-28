use crate::{Aggregatable, Aggregate, EString, ParseFragment};

/// Aggregate struct, that can sum inner aggregatable [items](Aggregatable::items) if
/// [``Aggregatable::Item``] implements [``std::iter::Sum``](std::iter::Sum)
///
/// # Examples
///
/// ```rust
/// use estring::{Aggregate, EString, SepVec, Sum};
/// let res = EString::from("1+2+3+4")
///     .parse::<Sum<SepVec<i32, '+'>>>()
///     .unwrap()
///     .agg();
/// assert_eq!(res, 10);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Sum<T>(pub T);

impl<T> ParseFragment for Sum<T>
where
    T: ParseFragment,
{
    fn parse_frag(es: EString) -> crate::Result<Self> {
        T::parse_frag(es).map(Self)
    }
}

impl<R, T> Aggregate for Sum<T>
where
    R: std::iter::Sum,
    T: Aggregatable<Item = R>,
{
    type Target = R;

    fn agg(self) -> Self::Target {
        self.0.items().into_iter().sum()
    }
}

impl<R, T> Aggregatable for Sum<T>
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
        match es.parse::<Sum<CommaVec<i32>>>() {
            Ok(res) => assert_eq!(res, Sum(CommaVec::from(vec![1, 2, 3]))),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_aggregate_vector() {
        let es = EString::from("1,2,3");
        let expr = es.parse::<Sum<CommaVec<i32>>>().unwrap();
        assert_eq!(expr.agg(), 6);
    }

    #[test]
    fn should_aggregate_vector_with_inner_vector() {
        let es = EString::from("1+2,2,3");
        let expr = es.parse::<Sum<CommaVec<PlusVec<i32>>>>().unwrap();
        assert_eq!(expr.agg(), 8);
    }

    #[test]
    fn should_aggregate_vector_with_inner_aggregation() {
        let es = EString::from("1+2,2,3");
        let expr = es.parse::<Sum<CommaVec<Sum<PlusVec<i32>>>>>().unwrap();
        assert_eq!(expr.agg(), 8);
    }
}
