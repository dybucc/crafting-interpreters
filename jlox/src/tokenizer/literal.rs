use std::{borrow::Cow, convert::Infallible, str::FromStr};

use crate::tokenizer::Num;

#[derive(Debug)]
pub(crate) enum Lit {
    Str(Cow<'static, str>),
    Num(Num),
}

impl FromStr for Lit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Str(s.to_owned().into()))
    }
}
