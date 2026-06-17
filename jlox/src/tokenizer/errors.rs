use std::{error::Error, fmt::Debug};

use thiserror::Error;

mod syntax_error;

pub(crate) use self::syntax_error::SyntaxError;

#[derive(Debug, Error)]
#[error("io error while reading line {line}: `{inner}`")]
pub(crate) struct IoBound {
    pub(crate) inner: Box<dyn Error + Send + Sync>,
    pub(crate) line: usize
}
