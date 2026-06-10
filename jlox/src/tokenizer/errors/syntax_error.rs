use std::{
    cmp::Ordering,
    error::Error,
    hash::{Hash, Hasher}
};

use anyhow::anyhow;
use thiserror::Error;

use crate::tokenizer::Location;

#[derive(Debug, Error)]
#[error("{span}: {src}")]
pub(crate) struct SyntaxError {
    pub(crate) span: Location,
    pub(crate) src: Box<dyn Error + Send + Sync + 'static>
}

impl SyntaxError {
    pub(crate) fn new(loc: Location, err: impl Error + Send + Sync + 'static) -> Self {
        Self { span: loc, src: Box::new(err) as Box<dyn Error + Send + Sync + 'static> }
    }

    pub(crate) fn other(loc: Location, err: impl Error + Send + Sync + 'static) -> Self {
        Self { span: loc, src: Box::new(err) as Box<dyn Error + Send + Sync + 'static> }
    }

    pub(crate) fn hash_with_err(&self, state: &mut impl Hasher) -> anyhow::Result<()> {
        self.span.hash(state);

        ErrorKind::which(self.src).ok_or_else(|| anyhow!("PENDING")).map(|err| match err {
            ErrorKind::MalformedNumber => todo!(),
            ErrorKind::UnexpectedEof => todo!(),
            ErrorKind::InvalidUtf8 => todo!(),
            ErrorKind::Other => todo!()
        })
    }

    pub(crate) fn same_line(&self, other: &Self) -> bool {
        let SyntaxError { span: src, .. } = self;
        let SyntaxError { span: other, .. } = other;

        src.same_line(other)
    }

    pub(crate) fn akin_spans(&self, other: &Self) -> bool {
        let SyntaxError { span: src, .. } = self;
        let SyntaxError { span: other, .. } = other;

        src.same_line(other) && src.akin_col(other)
    }

    pub(crate) fn merge(&mut self, other: &Self) {
        let SyntaxError { span: src, .. } = self;
        let SyntaxError { span: other, .. } = other;

        src.merge_cols(*other);
    }
}

/// This implemenation will hash on the error span only.
///
/// For an implementation that hashes on both the span and the underlying error,
/// see [`SyntaxError::hash_with_err`].
///
/// [`SyntaxError::hash_with_err`]: self::SyntaxError::hash_with_err
impl Hash for SyntaxError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.span.hash(state); }
}

impl PartialEq for SyntaxError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other).expect("internal error did not contain syntax errors"),
            Ordering::Equal
        )
    }
}

impl Eq for SyntaxError {}

impl PartialOrd for SyntaxError {
    /// The implementation orders first by span and then by concrete error type.
    ///
    /// Downcasting the error is left for last because it requires multiple runs
    /// of [`Error::downcast`] method on all possible syntax errors.
    ///
    /// [`Error::downcast`]: anyhow::Error::downcast
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.span.cmp(&other.span) {
            Ordering::Equal => ErrorKind::which(self.src).and_then(|self_err| {
                ErrorKind::which(other.src).map(|other_err| self_err.cmp(&other_err))
            }),
            other => other.into()
        }
    }
}

/// This macro ensures all syntax-related errors are defined in terms of a unit
/// record.
///
/// The macro expands to both the records with automatically derived `Error` and
/// `Display` implementations. It also provides the `ErrorKind` type, which
/// checks if any two given errors are equivalent or not.
macro_rules! errors {
    ($err:expr, $first:tt, $($others:tt),+ $(,)?) => {
        if $err.is::<$first>() {
            return Self::$first.into();
        }
        $(
            else if $err.is::<$others>() {
                return Self::$others.into();
            }
        )+

        None
    };
    ($($it:ident => $display:expr),+ $(,)?) => {
        #[derive(Debug, Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
        enum ErrorKind {$(
            $it,
        )+}

        impl ErrorKind {
            fn which(err: Box<dyn Error + Send + Sync + 'static>) -> Option<Self> {
                errors! { err, $($it),+ }
            }

            // TODO: check if we can return a less opaque type here that can
            // yield more information on a certain trait of the underlying (now
            // type-erased) type.
            fn which_err(err: Box<dyn Error + Send + Sync + 'static>) -> Option<()> {
                todo!()
            }
        }

        $(
            #[derive(Debug, Error, Copy, Hash)]
            #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
            #[error($display)]
            pub(crate) struct $it;
        )+
    };
}

errors! {
    MalformedNumber => "malformed number",
    UnexpectedEof => "unexpected eof before end of tokenization",
    InvalidUtf8 => "found invalid utf-8",
    Other => "unexpected bytes",
}
