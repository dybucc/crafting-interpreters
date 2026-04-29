use jlox::{errors::ErrorTrace, tokenizer::Location};
use thiserror::Error;

use crate::ToError;

#[derive(Debug, Error)]
#[error("PENDING")]
pub(crate) struct SyntaxError {
    pub(crate) span: Location,
    pub(crate) src: Box<dyn ErrorTrace>,
}

// TODO: keep working on embedding other error types within `SyntaxError`s.
impl SyntaxError {
    #[inline]
    pub(crate) fn new(loc: Location) -> Self {
        Self {
            span: loc,
            src: Box::new(Other.convert(None)) as Box<dyn ErrorTrace>,
        }
    }

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

#[derive(Debug, Error)]
#[error("unexpected byte(s)")]
pub(crate) struct Other;

#[derive(Debug, Error)]
#[error("unexpected eof before end of tokenization")]
pub(crate) struct UnexpectedEof;

#[derive(Debug, Error)]
#[error("found invalid utf-8")]
pub(crate) struct InvalidUtf8;
