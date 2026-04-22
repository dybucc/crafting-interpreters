use std::{borrow::Cow, error::Error, fmt::Debug};

use thiserror::Error;

macro_rules! error_impl {
    ($($t:ident),+) => {
        $(
            impl $crate::errors::ErrorTrace for $t {}
            impl $crate::errors::ToError for $t {}
        )+
    };
}

#[derive(Debug, Error)]
#[error(
    "syntax error at: {line}:{col}{}",
    if let Some(expect) = .expect {
        expect.info()
    } else {
        "".into()
    }
)]
pub(crate) struct SyntaxError {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) expect: Option<Box<dyn ExpectInfo + Send + Sync>>,
}

pub(crate) trait ExpectInfo: Debug {
    fn info(&self) -> Cow<'static, str>;
}

#[derive(Debug, Error)]
#[error("unexpected eof before tokenization completed")]
pub(crate) struct UnexpectedEof;

#[derive(Debug, Error)]
#[error("io error while reading line {line}: {inner}")]
pub(crate) struct IoBound {
    pub(crate) inner: Box<dyn Error + Send + Sync>,
    pub(crate) line: usize,
}

#[derive(Debug, Error)]
#[error("found invalid utf-8 at: {line}:{col}")]
pub(crate) struct InvalidUtf8 {
    pub(crate) line: usize,
    pub(crate) col: usize,
}

error_impl!(UnexpectedEof, IoBound, InvalidUtf8, SyntaxError);
