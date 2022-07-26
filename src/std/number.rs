use crate::core::{EString, ParseFragment};
use crate::error::{Error, Reason};

#[doc(hidden)]
macro_rules! from_env_string_numbers_impl {
    ($($ty:ty),+$(,)?) => {
        $(
            impl ParseFragment for $ty {
                #[inline]
                fn parse_frag(s: EString) -> crate::Result<Self> {
                    s.0.parse::<Self>().map_err(|_| Error(s, Reason::Parse))
                }
            }
        )+
    };
}

#[rustfmt::skip]
from_env_string_numbers_impl![
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_number() {
        let estr = EString::from("-10");
        match estr.parse::<i32>() {
            Ok(res) => assert_eq!(res, -10),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_float_number() {
        let estr = EString::from("-0.15");
        match estr.parse::<f32>() {
            #[allow(clippy::float_cmp)]
            Ok(res) => assert_eq!(res, -0.15),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_parse_error() {
        let estr = EString::from("-10");
        match estr.parse::<u32>() {
            Err(Error(orig, reason)) => {
                assert_eq!(orig, EString::from("-10"));
                assert_eq!(reason, Reason::Parse);
            }
            _ => unreachable!(),
        };
    }
}
