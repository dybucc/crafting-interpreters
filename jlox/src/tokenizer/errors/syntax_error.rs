use thiserror::Error;

use crate::{error_rt::ErrorTrace, tokenizer::Location};

#[derive(Debug, Error)]
#[error("{src}")]
pub(crate) struct SyntaxError {
    pub(crate) span: Location,
    pub(crate) src: Box<dyn ErrorTrace>,
}

impl SyntaxError {
    #[inline]
    pub(crate) fn new(loc: Location, err: Box<dyn ErrorTrace>) -> Self {
        Self {
            span: loc,
            src: err,
        }
    }

    #[inline]
    pub(crate) fn new_generic(loc: Location) -> Self {
        Self {
            span: loc,
            src: Box::new(Other) as Box<dyn ErrorTrace>,
        }
    }

    #[inline]
    pub(crate) fn same_line(&self, other: &Self) -> bool {
        let SyntaxError { span: src, .. } = self;
        let SyntaxError { span: other, .. } = other;

        src.same_line(other)
    }

    #[inline]
    pub(crate) fn akin_spans(&self, other: &Self) -> bool {
        let SyntaxError { span: src, .. } = self;
        let SyntaxError { span: other, .. } = other;

        src.akin_col(other)
    }

    #[inline]
    pub(crate) fn merge(&mut self, other: &Self) {
        let SyntaxError { span: src, .. } = self;
        let SyntaxError { span: other, .. } = other;

        src.merge_cols(*other);
    }
}

// NOTE: the following are intentionally coarse and do not provide further
// details on the error beyond what's obvious from the identifier, but that is
// intentional because they serve as the kinds of errors and thus as the string
// representation of the error within `SyntaxError`.

#[derive(Debug, Error)]
#[error("malforemd number")]
pub(crate) struct MalformedNumber;

#[derive(Debug, Error)]
#[error("unexpected byte(s)")]
pub(crate) struct Other;

#[derive(Debug, Error)]
#[error("unexpected eof before end of tokenization")]
pub(crate) struct UnexpectedEof;

#[derive(Debug, Error)]
#[error("found invalid utf-8")]
pub(crate) struct InvalidUtf8;
