use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

use thiserror::Error;

mod syntax_error;

pub(crate) use crate::tokenizer::errors::syntax_error::SyntaxError;

pub(crate) trait TokenizerError: Error {
    #[inline]
    fn inner(&self) -> Box<dyn Display> {
        Box::new(fmt::from_fn(|f| Display::fmt(self, f))) as Box<dyn Display>
    }
}

#[derive(Debug, Error)]
#[error("io error while reading line {line}: `{inner}`")]
pub(crate) struct IoBound {
    pub(crate) inner: Box<dyn Error + Send + Sync>,
    pub(crate) line: usize,
}
