//! Contains the implementations to parse triple-tuple type
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

/// Wrapper for trio (A, B, C) tuple to split string by separators (`S1` and `S2`).
///
/// **NOTE**: Required the enabling of the `tuple` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{Trio, EString};
///
/// type EqTrio<A, B, C> = Trio<A, '=', B, '=', C>;
///
/// fn main() -> Result<(), estring::ParseError> {
///     let res = EString::from("one=two=free").parse::<EqTrio<&str, &str, &str>>()?;
///     assert_eq!(res, Trio("one", "two", "free"));
///     Ok(())
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Trio<A, const S1: char, B, const S2: char, C>(pub A, pub B, pub C);

impl<A, B, C, const S1: char, const S2: char> From<(A, B, C)> for Trio<A, S1, B, S2, C> {
    #[inline]
    fn from((a, b, c): (A, B, C)) -> Self {
        Self(a, b, c)
    }
}

impl<A, B, C, const S1: char, const S2: char> std::fmt::Display for Trio<A, S1, B, S2, C>
where
    A: std::fmt::Display,
    B: std::fmt::Display,
    C: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())?;
        f.write_char(S1)?;
        f.write_str(&self.1.to_string())?;
        f.write_char(S2)?;
        f.write_str(&self.2.to_string())
    }
}

impl<A, const S1: char, B, const S2: char, C> TryFrom<EString> for Trio<A, S1, B, S2, C>
where
    A: TryFrom<EString>,
    B: TryFrom<EString>,
    C: TryFrom<EString>,
{
    type Error = Error;

    fn try_from(value: EString) -> Result<Self, Self::Error> {
        value.split_once(S1).ok_or(Error::Split).and_then(|(a, b)| {
            let a = A::try_from(EString::from(a)).map_err(|_| Error::Parse(0))?;
            b.split_once(S2).ok_or(Error::Split).and_then(|(b, c)| {
                let b = B::try_from(EString::from(b)).map_err(|_| Error::Parse(1))?;
                let c = C::try_from(EString::from(c)).map_err(|_| Error::Parse(2))?;
                Ok(Self(a, b, c))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::SepVec;

    #[test]
    fn should_parse_into_trio() {
        type EqTrio<A, B, C> = Trio<A, '=', B, '=', C>;
        let estr = EString::from("hello=world=hello");
        match estr.parse::<EqTrio<&str, &str, &str>>() {
            Ok(res) => assert_eq!((res.0, res.1, res.2), ("hello", "world", "hello")),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_into_trio_with_alternate_delims() {
        type EqTrio<A, B, C> = Trio<A, '-', B, '^', C>;
        let estr = EString::from("hello-world^hello");
        match estr.parse::<EqTrio<&str, &str, &str>>() {
            Ok(res) => assert_eq!((res.0, res.1, res.2), ("hello", "world", "hello")),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_rest_as_trio() {
        type EqTrio<A, B, C> = Trio<A, '=', B, '=', C>;
        let estr = EString::from("hello=world=hello=world=hello");
        match estr.parse::<EqTrio<&str, &str, EqTrio<&str, &str, &str>>>() {
            Ok(res) => assert_eq!(res, Trio("hello", "world", Trio("hello", "world", "hello"))),
            _ => unreachable!(),
        };
    }
}
