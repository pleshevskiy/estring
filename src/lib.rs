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
//! fn main() -> Result<(), estring::ParseError> {
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
pub use crate::error::ParseError;
