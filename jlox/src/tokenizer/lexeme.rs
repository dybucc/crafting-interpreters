use std::{borrow::Cow, convert::Infallible, str::FromStr};

#[derive(Debug)]
pub(crate) struct Lexeme {
    repr: LexemeRepr<'static>,
}

impl From<&[u8]> for Lexeme {
    fn from(value: &[u8]) -> Self {
        Self {
            repr: LexemeRepr::from(value),
        }
    }
}

impl FromStr for Lexeme {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            repr: LexemeRepr::from(s),
        })
    }
}

#[derive(Debug)]
struct LexemeRepr<'a> {
    inner: Cow<'a, [u8]>,
}

impl<B: AsRef<[u8]>> From<B> for LexemeRepr<'_> {
    fn from(value: B) -> Self {
        Self {
            inner: value.as_ref().to_owned().into(),
        }
    }
}
