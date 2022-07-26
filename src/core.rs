//! Contains the ``EString`` type, as well as the basic implementation of conversions to
//! string types
//!

// TODO: add more info and examples.
/// Format a value fragment into a ``EString``.
pub trait FormatFragment {
    /// Format this type and returns ``EString``.
    fn fmt_frag(&self) -> EString;
}

/// Parse a value fragment from a ``EString``.
///
/// ``ParseFragment``’s `parse_frag` method is often used implicitly, through ``EString``’s parse.
/// See [parse](EString::parse)’s documentation for examples.
///
/// # Examples
///
/// Basic implementation of ``ParseFragment`` on an example ``Point``.
///
/// ```rust
/// use estring::{EString, ParseFragment, Reason};
///
/// #[derive(Debug, PartialEq)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// impl ParseFragment for Point {
///     fn parse_frag(es: EString) -> estring::Result<Self> {
///         let orig = es.clone();
///         let (x, y) = es
///             .trim_matches(|p| p == '(' || p == ')')
///             .split_once(',')
///             .ok_or(estring::Error(orig, Reason::Split))?;
///
///         let (x, y) = (EString::from(x), EString::from(y));
///         let x = x.clone().parse::<i32>()
///             .map_err(|_| estring::Error(x, Reason::Parse))?;
///         let y = y.clone().parse::<i32>()
///             .map_err(|_| estring::Error(y, Reason::Parse))?;
///
///         Ok(Point { x, y })
///     }
/// }
///
/// let fragment = EString::from("(1,2)");
/// let res = Point::parse_frag(fragment).unwrap();
/// assert_eq!(res, Point { x: 1, y: 2 })
/// ```
///
pub trait ParseFragment: Sized {
    /// Parses a ``EString`` fragment `es` to return a value of this type.
    ///
    /// # Errors
    ///
    /// If parsing is not succeeds, returns ``Error`` inside ``Err`` with original fragment `es`
    /// and reason ``Reason``.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use estring::{EString, ParseFragment};
    ///
    /// let fragment = EString::from("5");
    /// let res = i32::parse_frag(fragment).unwrap();
    /// assert_eq!(res, 5);
    /// ```
    fn parse_frag(es: EString) -> crate::Result<Self>;
}

/// Wrapper under String type.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct EString(pub String);

impl EString {
    /// Parses this inner string into another type.
    ///
    /// `parse` can parse into any type that implements the ``ParseFragment`` trait.
    ///
    /// # Errors
    ///
    /// Will return `Err` if estring cannot parse inner fragment into the desired type.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// # use estring::{EString, ParseFragment};
    /// let fragment = EString::from("5");
    /// let res = i32::parse_frag(fragment);
    /// assert_eq!(res, Ok(5));
    /// ```
    ///
    /// Failing to parse:
    ///
    /// ```rust
    /// # use estring::{EString, ParseFragment, Error, Reason};
    /// let fragment = EString::from("j");
    /// let res = i32::parse_frag(fragment.clone());
    /// assert_eq!(res, Err(Error(fragment, Reason::Parse)));
    /// ```
    #[inline]
    pub fn parse<T: ParseFragment>(self) -> crate::Result<T> {
        T::parse_frag(self)
    }
}

impl<T> From<T> for EString
where
    T: std::fmt::Display,
{
    #[inline]
    fn from(val: T) -> Self {
        Self(val.to_string())
    }
}

impl std::ops::Deref for EString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ParseFragment for EString {
    #[inline]
    fn parse_frag(value: EString) -> crate::Result<Self> {
        Ok(value)
    }
}

impl ParseFragment for String {
    #[inline]
    fn parse_frag(s: EString) -> crate::Result<Self> {
        Ok(s.0)
    }
}

impl ParseFragment for &'static str {
    #[inline]
    fn parse_frag(s: EString) -> crate::Result<Self> {
        Ok(Box::leak(s.0.into_boxed_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deref_to_string() {
        let estr = EString::from("hello");
        assert_eq!(*estr, String::from("hello"));
    }

    #[test]
    fn should_parse_into_itself() {
        let estr = EString::from("hello");
        match estr.parse::<EString>() {
            Ok(res) => assert_eq!(res, EString::from("hello")),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_parse_into_string() {
        let estr = EString::from("hello");
        match estr.parse::<String>() {
            Ok(res) => assert_eq!(res, String::from("hello")),
            _ => unreachable!(),
        }
    }
}
