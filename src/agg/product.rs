use crate::{Aggregatable, Aggregate, EString, ParseFragment};

/// Aggregate struct, that can multiply inner aggregatable [items](Aggregatable::items) if
/// [``Aggregatable::Item``] implements [``std::iter::Product``](std::iter::Product)
///
/// # Examples
///
/// ```rust
/// use estring::{Aggregate, EString, SepVec, Product};
/// let res = EString::from("1*2*3*4")
///     .parse::<Product<SepVec<i32, '*'>>>()
///     .unwrap()
///     .agg();
/// assert_eq!(res, 24);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Product<T>(pub T);

impl<T> ParseFragment for Product<T>
where
    T: ParseFragment,
{
    fn parse_frag(es: EString) -> crate::Result<Self> {
        T::parse_frag(es).map(Self)
    }
}

impl<R, T> Aggregate for Product<T>
where
    R: std::iter::Product,
    T: Aggregatable<Item = R>,
{
    type Target = R;

    fn agg(self) -> Self::Target {
        self.0.items().into_iter().product()
    }
}

impl<R, T> Aggregatable for Product<T>
where
    R: std::iter::Product,
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
    type MulVec<T> = SepVec<T, '*'>;

    #[test]
    fn should_parse_vec() {
        let es = EString::from("1,2,3");
        match es.parse::<Product<CommaVec<i32>>>() {
            Ok(res) => assert_eq!(res, Product(CommaVec::from(vec![1, 2, 3]))),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_aggregate_vector() {
        let es = EString::from("1,2,3");
        let expr = es.parse::<Product<CommaVec<i32>>>().unwrap();
        assert_eq!(expr.agg(), 6);
    }

    #[test]
    fn should_aggregate_vector_with_inner_vector() {
        let es = EString::from("1*2,2,3");
        let expr = es.parse::<Product<CommaVec<MulVec<i32>>>>().unwrap();
        assert_eq!(expr.agg(), 12);
    }

    #[test]
    fn should_aggregate_vector_with_inner_aggregation() {
        let es = EString::from("1*2,2,3");
        let expr = es
            .parse::<Product<CommaVec<Product<MulVec<i32>>>>>()
            .unwrap();
        assert_eq!(expr.agg(), 12);
    }
}
