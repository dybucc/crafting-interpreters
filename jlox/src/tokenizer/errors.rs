use std::{
    error::Error,
    fmt::{Debug, Display}
};

use thiserror::Error;

mod syntax_error;

pub(crate) use self::syntax_error::SyntaxError;

pub(crate) trait TokenizerError: Error {
    #[inline]
    fn inner(&self) -> Box<dyn Display> { Box::new(self) as Box<dyn Display> }
}

pub(crate) trait OtherError: Error {
    #[inline]
    fn inner(&self) -> Box<dyn Display> { Box::new(self) as Box<dyn Display> }
}

#[derive(Debug, Error)]
#[error("io error while reading line {line}: `{inner}`")]
pub(crate) struct IoBound {
    pub(crate) inner: Box<dyn Error + Send + Sync>,
    pub(crate) line: usize
}

impl OtherError for IoBound {}
