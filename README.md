# EString

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/pleshevskiy/estring/CI?label=tests&logo=github&style=flat-square)](https://github.com/pleshevskiy/estring/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/estring?style=flat-square)](https://docs.rs/estring)
[![Crates.io](https://img.shields.io/crates/v/estring?style=flat-square)](https://crates.io/crates/estring)
[![Crates.io](https://img.shields.io/crates/l/estring?style=flat-square)](https://github.com/pleshevskiy/estring/LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square)](https://github.com/rust-secure-code/safety-dance/)
[![Matrix](https://img.shields.io/matrix/enve_team:matrix.org?label=matrix&style=flat-square)](https://matrix.to/#/!yZalHbWfGRWOMaetSn:matrix.org?via=matrix.org)

A simple way to parse a string using type annotations.

This package was originally designed for [enve]

[enve]: https://github.com/pleshevskiy/enve

## Getting started

```rust
use estring::{SepVec, EString};

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> Result<(), estring::ParseError> {
    let res = EString::from("10+5*2+3")
        .parse::<PlusVec<MulVec<f32>>>()?
        .iter()
        .map(|m| m.iter().product::<f32>())
        .sum::<f32>();

    assert_eq!(res, 23.0);
    Ok(())
}
```

You can use custom types as annotations! Just implement `TryFrom<EString>`!

## Installation

**The MSRV is 1.59.0**

Add `estring = { version = "0.1", features = ["vec", "number"] }` as a
dependency in `Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
estring = { version = "0.1", features = ["vec", "number"] }
```

## License

**MIT**. See [LICENSE](https://github.com/pleshevskiy/estring/LICENSE) to see
the full text.

## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator,
maintainer.
