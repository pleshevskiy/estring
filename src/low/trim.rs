use crate::core::{EString, ParseFragment, ToEString};

/// Wrapper that allow to trim substring before continue
///
/// **NOTE**: Required the enabling of the `low-level` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{EString, Trim};
///
/// fn main() -> estring::Result<()> {
///     let res = EString::from(" 99 ").parse::<Trim<i32>>()?;
///     assert_eq!(res, Trim(99));
///     Ok(())
/// }
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct Trim<T>(pub T);

impl<T> std::ops::Deref for Trim<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Trim<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> ParseFragment for Trim<T>
where
    T: ParseFragment,
{
    fn parse_frag(value: EString) -> crate::Result<Self> {
        T::parse_frag(EString::from(value.trim())).map(Trim)
    }
}

impl<T> ToEString for Trim<T>
where
    T: ToEString,
{
    fn to_estring(&self) -> EString {
        self.0.to_estring()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_trim_string() {
        let estr = EString::from("    999   ");

        match estr.parse::<Trim<&str>>() {
            Ok(res) => assert_eq!(*res, "999"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_trim_and_convert_to_number() {
        let estr = EString::from("    999   ");

        match estr.parse::<Trim<i32>>() {
            Ok(res) => assert_eq!(*res, 999),
            _ => unreachable!(),
        }
    }
}
