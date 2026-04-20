use std::error::Error;

use thiserror::Error;

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

macro_rules! error_impl {
    ($($t:ident),+) => {
        $(
            impl $crate::errors::ErrorTrace for $t {}
            impl $crate::errors::ToError for $t {}
        )+
    };
}

error_impl!(UnexpectedEof, IoBound, InvalidUtf8);
