//! Contains the ``EString`` type, as well as the basic implementation of conversions to
//! string types
//!

/// Format this type and wrap into ``EString``.
///
/// ``ToEString``’s `to_estring` method is often used implicitly, through ``EString``’s from.
///
/// # Examples
///
/// Basic implementation of ``ToEString`` on an example ``Point``.
///
/// ```rust
/// use std::fmt::Write;
/// use estring::{EString, ToEString};
///
/// #[derive(Debug, PartialEq)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// impl ToEString for Point {
///     fn to_estring(&self) -> EString {
///         let mut res = String::new();
///         write!(res, "({},{})", self.x, self.y)
///             .ok()
///             .expect("Cannot format Point into EString");
///         EString(res)
///     }
/// }
///
/// let point = Point { x: 1, y: 2 };
/// assert_eq!(point.to_estring(), EString::from("(1,2)"));
/// ```
///
pub trait ToEString {
    /// Format this type and returns ``EString``.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use estring::{EString, ToEString};
    ///
    /// let i = 5;
    /// let five = EString::from(5);
    /// assert_eq!(five, i.to_estring());
    /// ```
    fn to_estring(&self) -> EString;
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

/// Wrapper under ``String`` type.
///
/// # Examples
///
/// You can create a ``EString`` from a any type that implement ``ToEString`` with ``EString::from``
///
/// ```rust
/// # use estring::EString;
/// let hello = EString::from("Hello, world");
/// let num = EString::from("999");
/// ```
///
/// You can use ``ToEString::to_estring`` directly on the type.
///
/// ```rust
/// # use estring::ToEString;
/// let some_opt = Some(999).to_estring();
/// let none_opt = None::<i32>.to_estring();
/// ```
///
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct EString(pub String);

impl std::fmt::Display for EString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EString {
    /// Creates a new empty ``EString``.
    ///
    /// This will not allocate any inital buffer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use estring::EString;
    /// let s = EString::new();
    /// ```
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self(String::new())
    }

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
    T: ToEString,
{
    #[inline]
    fn from(val: T) -> Self {
        val.to_estring()
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
    fn parse_frag(es: EString) -> crate::Result<Self> {
        Ok(es)
    }
}

impl ParseFragment for String {
    #[inline]
    fn parse_frag(es: EString) -> crate::Result<Self> {
        Ok(es.0)
    }
}

impl ToEString for String {
    #[inline]
    fn to_estring(&self) -> EString {
        EString(self.clone())
    }
}

impl ParseFragment for &'static str {
    #[inline]
    fn parse_frag(es: EString) -> crate::Result<Self> {
        Ok(Box::leak(es.0.into_boxed_str()))
    }
}

impl<'a> ToEString for &'a str {
    #[inline]
    fn to_estring(&self) -> EString {
        EString((*self).to_string())
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
