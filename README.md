# EString

[![Crates.io](https://img.shields.io/crates/v/estring?style=flat-square)](https://crates.io/crates/estring)
[![docs.rs](https://img.shields.io/docsrs/estring?style=flat-square)](https://docs.rs/estring)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/pleshevskiy/estring/CI?label=tests&logo=github&style=flat-square)](https://github.com/pleshevskiy/estring/actions/workflows/ci.yml)
![The MSRV](https://img.shields.io/badge/MSRV-1.59.0-red.svg)

```toml
[dependencies]
estring = "0.2"
```

A simple way to parse a string using type annotations.

This package was originally designed for [enve].

[enve]: https://github.com/pleshevskiy/enve

## [Documentation](https://docs.rs/estring)

For more details, see [examples].

[examples]: https://github.com/pleshevskiy/estring/tree/main/examples

## Usage

```rust
use estring::{SepVec, EString};

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> estring::Result<()> {
    let res = EString::from("10+5*2+3")
        .parse::<PlusVec<MulVec<f32>>>()?
        .iter()
        .map(|m| m.iter().product::<f32>())
        .sum::<f32>();

    assert_eq!(res, 23.0);
    Ok(())
}
```

You can use custom types as annotations! Just implement
`estring::ParseFragment`!

## Contact Us

Join us in:

[![Matrix](https://img.shields.io/badge/matrix-%23enve_team:matrix.org-blueviolet.svg?style=flat-square)](https://matrix.to/#/#enve_team:matrix.org)

## License

**MIT**. See [LICENSE](https://github.com/pleshevskiy/estring/LICENSE) to see
the full text.

## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator,
maintainer.
