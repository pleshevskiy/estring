//!
//! # ``EString``
//!
//! A simple way to parse a string using type annotations.
//!
//! This package was originally designed for [enve]
//!
//! [enve]: https://github.com/pleshevskiy/enve
//!
//! ## Getting started
//!
//! ```rust
//! use estring::{SepVec, EString};
//!
//! type PlusVec<T> = SepVec<T, '+'>;
//! type MulVec<T> = SepVec<T, '*'>;
//!
//! fn main() -> estring::Result<()> {
//!     let res = EString::from("10+5*2+3")
//!         .parse::<PlusVec<MulVec<f32>>>()?
//!         .iter()
//!         .map(|m| m.iter().product::<f32>())
//!         .sum::<f32>();
//!
//!     assert_eq!(res, 23.0);
//!     Ok(())
//! }
//! ```
//!
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]

mod error;
pub use error::{Error, Reason};
/// The type returned by parser methods.
///
/// # Examples
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
pub type Result<T> = ::std::result::Result<T, Error>;

pub mod core;
pub mod std;

#[cfg(feature = "aggs")]
pub mod agg;
#[cfg(feature = "aggs")]
pub use agg::*;

#[cfg(feature = "low-level")]
pub mod low;
#[cfg(feature = "low-level")]
pub use low::*;
#[cfg(feature = "structs")]
pub mod structs;
#[cfg(feature = "structs")]
pub use structs::*;

pub use crate::core::*;
