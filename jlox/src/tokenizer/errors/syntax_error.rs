use std::{
    any::TypeId,
    cmp::Ordering,
    error::Error,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    marker::PhantomData
};

use thiserror::Error;

use crate::tokenizer::Location;

const trait SyntaxErrorMarker: Display {
    fn new() -> Self;
}

#[derive(Debug, Error)]
#[error("{span}: {}", E::new())]
pub(crate) struct SyntaxError<E: SyntaxErrorMarker> {
    span: Location,
    _marker: PhantomData<E>
}

impl<E: SyntaxErrorMarker> SyntaxError<E> {
    pub(crate) fn new(loc: Location, err: E) -> Self { Self { span: loc, _marker: PhantomData } }

    pub(crate) fn same_line(&self, other: &Self) -> bool { self.span.same_line(other.span) }

    pub(crate) fn akin_spans(&self, other: &Self) -> bool {
        self.span.same_line(other.span) && self.span.akin_col(other.span)
    }

    pub(crate) fn merge(&mut self, other: &Self) { self.span.merge_cols(other.span); }
}

impl<E: SyntaxErrorMarker + Hash> Hash for SyntaxError<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.span.hash(state);

        E::new().hash(state)
    }
}

impl<E1: SyntaxErrorMarker + PartialEq, E2: SyntaxErrorMarker + PartialEq>
    PartialEq<SyntaxError<E2>> for SyntaxError<E1>
{
    fn eq(&self, other: &SyntaxError<E2>) -> bool {
        self.span.eq(&other.span) && E1::new().eq(&E2::new())
    }
}

impl Eq for SyntaxError {}

impl PartialOrd for SyntaxError {
    /// The implementation orders first by span and then by concrete error type.
    ///
    /// Downcasting the error is left for last because it requires multiple runs
    /// of [`Error::downcast`] method on all possible syntax errors.
    ///
    /// [`Error::downcast`]: fn@anyhow::Error::downcast
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

        const impl ErrorKind {
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
            #[derive_const(Clone)]
            #[error($display)]
            pub(crate) struct $it;

            const impl SyntaxErrorMarker for $it {
                fn new() -> Self { Self }
            }

            const impl<E: SyntaxErrorMarker> PartialEq<E> for $it {
                fn eq(&self, other: &E) -> bool { TypeId::of::<Self>().eq(&TypeId::of::<E>()) }
            }

            const impl Eq for $it {}
        )+
    };
}

errors! {
    MalformedNumber => "malformed number",
    UnexpectedEof => "unexpected eof before end of tokenization",
    InvalidUtf8 => "found invalid utf-8",
    Other => "unexpected bytes",
}
