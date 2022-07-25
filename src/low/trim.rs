use crate::core::EString;

/// Wrapper that allow to trim substring before continue
///
/// **NOTE**: Required the enabling of the `low-level` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{EString, Trim};
///
/// fn main() -> Result<(), estring::ParseError> {
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

impl<T> TryFrom<EString> for Trim<T>
where
    T: TryFrom<EString>,
{
    type Error = ();

    fn try_from(value: EString) -> Result<Self, Self::Error> {
        T::try_from(EString::from(value.trim()))
            .map(Trim)
            .map_err(|_| ())
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

    #[cfg(feature = "number")]
    #[test]
    fn should_trim_and_convert_to_number() {
        let estr = EString::from("    999   ");

        match estr.parse::<Trim<i32>>() {
            Ok(res) => assert_eq!(*res, 999),
            _ => unreachable!(),
        }
    }
}
