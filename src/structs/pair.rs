//! Contains the implementations to pair tuple type
//!

use crate::core::{EString, ParseFragment};
use crate::{Error, Reason};
use std::fmt::Write;

/// Wrapper for pair (A, B) tuple to split string by a separator (`S1`).
///
/// **NOTE**: Required the enabling of the `structs` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{Pair, EString};
///
/// type EqPair<A, B> = Pair<A, '=', B>;
///
/// fn main() -> estring::Result<()> {
///     let res = EString::from("one=two=free").parse::<EqPair<&str, &str>>()?;
///     assert_eq!(res, Pair("one", "two=free"));
///     Ok(())
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Pair<A, const S1: char, B>(pub A, pub B);

impl<A, B, const S1: char> From<(A, B)> for Pair<A, S1, B> {
    #[inline]
    fn from((a, b): (A, B)) -> Self {
        Self(a, b)
    }
}

impl<A, B, const S1: char> std::fmt::Display for Pair<A, S1, B>
where
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())?;
        f.write_char(S1)?;
        f.write_str(&self.1.to_string())
    }
}

impl<A, const S1: char, B> ParseFragment for Pair<A, S1, B>
where
    A: ParseFragment,
    B: ParseFragment,
{
    fn parse_frag(value: EString) -> crate::Result<Self> {
        value
            .clone()
            .split_once(S1)
            .ok_or(Error(value, Reason::Split))
            .and_then(|(a, b)| {
                let (a, b) = (EString::from(a), EString::from(b));
                let a = A::parse_frag(a.clone()).map_err(|_| Error(a, Reason::Parse))?;
                let b = B::parse_frag(b.clone()).map_err(|_| Error(b, Reason::Parse))?;
                Ok(Self(a, b))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::SepVec;

    type EqPair<A, B> = Pair<A, '=', B>;

    #[test]
    fn should_parse_into_pair() {
        let estr = EString::from("hello=world=hello");
        match estr.parse::<EqPair<&str, &str>>() {
            Ok(res) => assert_eq!((res.0, res.1), ("hello", "world=hello")),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_rest_as_pair() {
        let estr = EString::from("hello=world=hello");
        match estr.parse::<EqPair<&str, EqPair<&str, &str>>>() {
            Ok(res) => assert_eq!(res, Pair("hello", Pair("world", "hello"))),
            _ => unreachable!(),
        };
    }

    type LineVec<T> = SepVec<T, '\n'>;

    #[test]
    fn should_parse_vec_of_pairs() {
        let estr = EString::from(
            "foo=bar
hello=bar",
        );
        match estr.parse::<LineVec<EqPair<&str, &str>>>() {
            Ok(res) => assert_eq!(res, SepVec(vec![Pair("foo", "bar"), Pair("hello", "bar"),])),
            _ => unreachable!(),
        };
    }
}
