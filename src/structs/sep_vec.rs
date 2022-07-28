//! Contains the implementations to vec type
//!

use crate::core::{EString, ParseFragment, ToEString};
use std::fmt::Write;

/// Wrapper for ``Vec`` to split string by a separator (`SEP`).
///
/// **NOTE**: Required the enabling of the `structs` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{SepVec, EString};
///
/// type CommaVec<T> = SepVec<T, ','>;
///
/// fn main() -> estring::Result<()> {
///     let res = EString::from("1,2,3").parse::<CommaVec<u8>>()?;
///     assert_eq!(*res, vec![1, 2, 3]);
///     Ok(())
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct SepVec<T, const SEP: char>(pub Vec<T>);

impl<T, const SEP: char> std::ops::Deref for SepVec<T, SEP> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const SEP: char> From<Vec<T>> for SepVec<T, SEP> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T, const SEP: char> std::fmt::Display for SepVec<T, SEP>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().enumerate().try_for_each(|(i, part)| {
            if i != 0 {
                f.write_char(SEP)?;
            }

            write!(f, "{}", part)
        })
    }
}

impl<T, const SEP: char> ToEString for SepVec<T, SEP>
where
    T: ToEString,
{
    fn to_estring(&self) -> EString {
        self.0
            .iter()
            .enumerate()
            .try_fold(String::new(), |mut res, (i, part)| {
                if i != 0 {
                    res.write_char(SEP).ok()?;
                }

                write!(res, "{}", part.to_estring()).ok()?;
                Some(res)
            })
            .map(EString)
            .expect("Cannot format SepVec ${self.0} to EString")
    }
}

impl<T, const SEP: char> ParseFragment for SepVec<T, SEP>
where
    T: ParseFragment,
{
    fn parse_frag(value: EString) -> crate::Result<Self> {
        let inner = value
            .split(SEP)
            .map(str::trim)
            .map(EString::from)
            .map(T::parse_frag)
            .collect::<crate::Result<Vec<_>>>()?;
        Ok(Self(inner))
    }
}

#[cfg(feature = "aggs")]
impl<T, const SEP: char> crate::core::Aggregatable for SepVec<T, SEP>
where
    T: crate::core::Aggregatable,
{
    type Item = T::Item;

    fn items(self) -> Vec<Self::Item> {
        self.0.into_iter().flat_map(T::items).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Aggregatable;
    use crate::Pair;
    use crate::{Error, Reason};

    type CommaVec<T> = SepVec<T, ','>;
    type SemiVec<T> = SepVec<T, ';'>;

    #[test]
    fn should_parse_into_vec() {
        let estr = EString::from("a,b,c,d,e");
        match estr.parse::<CommaVec<&str>>() {
            Ok(res) => assert_eq!(*res, vec!["a", "b", "c", "d", "e"]),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_trim_identations_before_parsing() {
        let input = "
a , b, c,
d,e";
        let estr = EString::from(input);
        match estr.parse::<CommaVec<&str>>() {
            Ok(res) => assert_eq!(*res, vec!["a", "b", "c", "d", "e"]),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_into_vector_of_vectors() {
        let estr = EString::from("a,b; c,d,e; f,g");
        match estr.parse::<SemiVec<CommaVec<&str>>>() {
            Ok(res) => assert_eq!(
                res,
                SemiVec::from(vec![
                    CommaVec::from(vec!["a", "b"]),
                    CommaVec::from(vec!["c", "d", "e"]),
                    CommaVec::from(vec!["f", "g"])
                ])
            ),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_into_num_vec() {
        let estr = EString::from("1,2,3,4,5");
        match estr.parse::<CommaVec<i32>>() {
            Ok(res) => assert_eq!(*res, vec![1, 2, 3, 4, 5]),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_parse_vec_error() {
        let estr = EString::from("1,2,3,4,5");
        match estr.parse::<SemiVec<i32>>() {
            Err(Error(orig, reason)) => {
                assert_eq!(orig, EString::from("1,2,3,4,5"));
                assert_eq!(reason, Reason::Parse);
            }
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_format_vec() {
        type PlusPair<T> = Pair<T, '+', T>;

        let vec = SepVec::<_, ','>::from(vec![1, 2, 3]);
        assert_eq!(vec.to_estring(), EString(String::from("1,2,3")));
        let vec = SepVec::<_, ','>::from(vec![PlusPair::from((1, 2)), PlusPair::from((3, 4))]);
        assert_eq!(vec.to_estring(), EString(String::from("1+2,3+4")));
    }

    #[test]
    fn should_returns_aggregatable_items() {
        let estr = EString::from("1,2,3,4,5");
        let res = estr.parse::<CommaVec<i32>>().unwrap();
        assert_eq!(res.items(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_returns_flatten_aggregatable_items() {
        let estr = EString::from("1,2; 3,4,5; 6,7");
        let res = estr.parse::<SemiVec<CommaVec<i32>>>().unwrap();
        assert_eq!(res.items(), vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
