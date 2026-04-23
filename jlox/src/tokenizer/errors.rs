use std::{borrow::Cow, error::Error, fmt::Debug};

use thiserror::Error;

use crate::tokenizer::Location;

macro_rules! error_trace_impl {
    ($($t:ident),+ $(,)?) => {
        $(
            impl $crate::errors::ErrorTrace for $t {}
            impl $crate::errors::ToError for $t {}
        )+
    };
}

#[derive(Debug, Error)]
#[error("pending")]
pub(crate) struct SyntaxError {
    pub(crate) repr: SyntaxErrorContainer,
}

pub(crate) type SyntaxErrorContainer = Vec<Location>;

#[derive(Debug, Error)]
#[error("unexpected eof before end of tokenization")]
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

error_trace_impl!(UnexpectedEof, IoBound, InvalidUtf8, SyntaxError);
