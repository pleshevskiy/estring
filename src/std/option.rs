use crate::core::{EString, ParseFragment};

impl<T> ParseFragment for Option<T>
where
    T: ParseFragment,
{
    fn parse_frag(es: EString) -> crate::Result<Self> {
        if es.is_empty() {
            Ok(None)
        } else {
            es.parse().map(Some)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_empty_string_as_none() {
        let estr = EString::from("");
        match estr.parse::<Option<i32>>() {
            Ok(res) => assert_eq!(res, None),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_parse_number_as_some() {
        let estr = EString::from("99");
        match estr.parse::<Option<i32>>() {
            Ok(res) => assert_eq!(res, Some(99)),
            _ => unreachable!(),
        }
    }
}
