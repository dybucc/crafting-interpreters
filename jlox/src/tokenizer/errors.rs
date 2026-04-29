use std::{error::Error, fmt::Debug};

use thiserror::Error;

mod syntax_error;

pub(crate) use crate::tokenizer::errors::syntax_error::{
    InvalidUtf8, Other, SyntaxError, UnexpectedEof,
};

macro_rules! error_trace_impl {
    ($($t:ident),+ $(,)?) => {
        $(
            impl $crate::errors::ErrorTrace for $t {}
            impl $crate::errors::ToError for $t {}
        )+
    };
}

#[derive(Debug, Error)]
#[error("io error while reading line {line}: {inner}")]
pub(crate) struct IoBound {
    pub(crate) inner: Box<dyn Error + Send + Sync>,
    pub(crate) line: usize,
}

error_trace_impl! {
    UnexpectedEof,
    IoBound,
    InvalidUtf8,
    SyntaxError,
    Other
}
