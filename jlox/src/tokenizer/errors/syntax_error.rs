use thiserror::Error;

use crate::tokenizer::Location;

#[derive(Debug, Error)]
#[error("pending")]
pub(crate) struct SyntaxError(pub(crate) Location);

impl SyntaxError {
    #[inline]
    pub(crate) fn new(loc: Location) -> Self {
        Self(loc)
    }
}
