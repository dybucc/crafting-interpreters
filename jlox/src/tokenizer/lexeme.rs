use std::{borrow::Cow, convert::Infallible, str::FromStr};

#[derive(Debug)]
pub(crate) struct Lexeme {
    repr: Cow<'static, [u8]>,
}

impl From<&[u8]> for Lexeme {
    fn from(value: &[u8]) -> Self {
        Self {
            repr: value.to_owned().into(),
        }
    }
}

impl FromStr for Lexeme {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            repr: s.as_bytes().to_owned().into(),
        })
    }
}
