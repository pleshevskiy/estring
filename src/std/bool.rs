use crate::core::{EString, ParseFragment, ToEString};
use crate::error::{Error, Reason};

impl ParseFragment for bool {
    #[inline]
    fn parse_frag(s: EString) -> crate::Result<Self> {
        match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
            "false" | "f" | "no" | "n" | "off" | "0" | "" => Ok(false),
            _ => Err(Error(s, Reason::Parse)),
        }
    }
}

impl ToEString for bool {
    #[inline]
    fn to_estring(&self) -> EString {
        EString(self.to_string())
    }
}

#[cfg(feature = "aggs")]
impl crate::core::Aggregatable for bool {
    type Item = Self;

    #[inline]
    fn items(self) -> Vec<Self::Item> {
        vec![self]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_bool_variable() {
        let test_cases = [
            ("1", true),
            ("0", false),
            ("y", true),
            ("f", false),
            ("yes", true),
            ("true", true),
            ("false", false),
            ("t", true),
            ("f", false),
            ("on", true),
            ("off", false),
        ];

        for (val, expected) in test_cases {
            let estr = EString::from(val);
            match estr.parse::<bool>() {
                Ok(res) => assert_eq!(res, expected),
                _ => unreachable!(),
            };
        }
    }

    #[test]
    fn should_throw_parse_error() {
        let estr = EString::from("something");
        match estr.parse::<bool>() {
            Err(crate::Error(orig, reason)) => {
                assert_eq!(orig, EString::from("something"));
                assert_eq!(reason, crate::Reason::Parse);
            }
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_format_bool() {
        assert_eq!(true.to_estring(), EString(String::from("true")));
        assert_eq!(false.to_estring(), EString(String::from("false")));
    }
}
