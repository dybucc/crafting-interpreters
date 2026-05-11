use std::{borrow::Cow, convert::Infallible, str::FromStr};

use crate::tokenizer::Num;

#[derive(Debug)]
pub(crate) struct Lit {
    repr: LitRepr,
}

impl FromStr for Lit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LitRepr::from_str(s).map(|repr| Self { repr })
    }
}

#[derive(Debug)]
enum LitRepr {
    Str(Str),
    Num(Num),
}

impl FromStr for LitRepr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Str::from_str(s).map(Self::Str)
    }
}

#[derive(Debug)]
struct Str {
    repr: StrRepr<'static>,
}

impl FromStr for Str {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            repr: StrRepr::new(s),
        })
    }
}

#[derive(Debug)]
struct StrRepr<'a> {
    inner: Cow<'a, str>,
}

impl StrRepr<'_> {
    fn new(s: impl AsRef<str>) -> Self {
        Self {
            inner: s.as_ref().to_owned().into(),
        }
    }
}
