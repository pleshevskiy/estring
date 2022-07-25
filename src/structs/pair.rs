//! Contains the implementations to pair tuple type
//!

use crate::core::EString;
use std::fmt::Write;

/// The error type for operations interacting with parsing tuples. Possibly returned from
/// ``EString::parse``
#[derive(Debug)]
pub enum Error {
    /// The specified input string is not split.
    Split,

    /// The specified substring of the split input string is not parsed
    Parse(u8),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Split => f.write_str("Cannot split input string"),
            Error::Parse(n) => write!(f, "Cannot parse {} substring", n),
        }
    }
}

impl std::error::Error for Error {}

/// Wrapper for pair (A, B) tuple to split string by a separator (`S1`).
///
/// **NOTE**: Required the enabling of the `tuple` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{Pair, EString};
///
/// type EqPair<A, B> = Pair<A, '=', B>;
///
/// fn main() -> Result<(), estring::ParseError> {
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

impl<A, const S1: char, B> TryFrom<EString> for Pair<A, S1, B>
where
    A: TryFrom<EString>,
    B: TryFrom<EString>,
{
    type Error = Error;

    fn try_from(value: EString) -> Result<Self, Self::Error> {
        value.split_once(S1).ok_or(Error::Split).and_then(|(a, b)| {
            let a = A::try_from(EString::from(a)).map_err(|_| Error::Parse(0))?;
            let b = B::try_from(EString::from(b)).map_err(|_| Error::Parse(1))?;
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
