//!
//! # ``EString``
//!
//! A simple way to parse a string using type annotations.
//!
//! This package was originally designed for [enve]
//!
//! [enve]: https://github.com/pleshevskiy/itconfig-rs/tree/redesign
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
#![deny(missing_docs)]

pub mod core;
mod error;

pub use crate::core::*;
pub use crate::error::ParseError;
