//! Contains the implementations to parse triple-tuple type
//!

use super::Pair;
use crate::core::{EString, ParseFragment, ToEString};
use std::fmt::Write;

/// Wrapper for trio (A, B, C) tuple to split string by separators (`S1` and `S2`).
///
/// **NOTE**: Required the enabling of the `structs` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{Trio, EString};
///
/// fn main() -> estring::Result<()> {
///     let res = EString::from("one+two=free").parse::<Trio<&str, '+', &str, '=', &str>>()?;
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

impl<A, B, C, const S1: char, const S2: char> ToEString for Trio<A, S1, B, S2, C>
where
    A: ToEString,
    B: ToEString,
    C: ToEString,
{
    fn to_estring(&self) -> EString {
        let mut res = String::new();
        write!(
            res,
            "{}{}{}{}{}",
            self.0.to_estring(),
            S1,
            self.1.to_estring(),
            S2,
            self.2.to_estring()
        )
        .ok()
        .expect("Cannot parse Pair to EString");
        EString(res)
    }
}

impl<A, const S1: char, B, const S2: char, C> ParseFragment for Trio<A, S1, B, S2, C>
where
    A: ParseFragment,
    B: ParseFragment,
    C: ParseFragment,
{
    fn parse_frag(value: EString) -> crate::Result<Self> {
        Pair::<A, S1, EString>::parse_frag(value).and_then(|Pair(a, rest)| {
            Pair::<B, S2, C>::parse_frag(rest).map(|Pair(b, c)| Self(a, b, c))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type EqTrio<A, B, C> = Trio<A, '=', B, '=', C>;

    #[test]
    fn should_parse_into_trio() {
        let estr = EString::from("hello=world=hello");
        match estr.parse::<EqTrio<&str, &str, &str>>() {
            Ok(res) => assert_eq!((res.0, res.1, res.2), ("hello", "world", "hello")),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_into_trio_with_alternate_delims() {
        let estr = EString::from("hello-world^hello");
        match estr.parse::<Trio<&str, '-', &str, '^', &str>>() {
            Ok(res) => assert_eq!((res.0, res.1, res.2), ("hello", "world", "hello")),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_rest_as_trio() {
        let estr = EString::from("hello=world=hello=world=hello");
        match estr.parse::<EqTrio<&str, &str, EqTrio<&str, &str, &str>>>() {
            Ok(res) => assert_eq!(res, Trio("hello", "world", Trio("hello", "world", "hello"))),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_format_trio() {
        let trio = Trio::<_, '+', _, '-', _>::from(("foo", "baz", "bar"));
        assert_eq!(
            trio.clone().to_estring(),
            EString(String::from("foo+baz-bar"))
        );

        let trio_in_trio = Trio::<_, '*', _, '=', _>::from(("foo", "baz", trio));
        assert_eq!(
            trio_in_trio.clone().to_estring(),
            EString(String::from("foo*baz=foo+baz-bar"))
        );
    }
}
