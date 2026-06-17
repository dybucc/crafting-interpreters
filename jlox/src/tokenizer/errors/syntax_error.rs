use std::{
    cmp::Ordering,
    error::Error,
    hash::{Hash, Hasher},
    marker::PhantomData
};

use thiserror::Error;

use crate::tokenizer::Location;

const trait SyntaxErrorMarker: Error {
    fn new() -> Self;
}

#[derive(Debug, Error)]
#[error("{span}: {}", E::new())]
pub(crate) struct SyntaxError<E: SyntaxErrorMarker> {
    span: Location,
    _marker: PhantomData<E>
}

impl<E: SyntaxErrorMarker> SyntaxError<E> {
    #[inline]
    pub(crate) fn new(loc: Location) -> Self { Self { span: loc, _marker: PhantomData } }

    #[inline]
    pub(crate) fn same_line(&self, other: &Self) -> bool { self.span.same_line(other.span) }

    #[inline]
    pub(crate) fn akin_spans(&self, other: &Self) -> bool {
        self.span.same_line(other.span) && self.span.akin_col(other.span)
    }

    #[inline]
    pub(crate) fn merge(&mut self, other: &Self) { self.span.merge_cols(other.span); }
}

impl<E: SyntaxErrorMarker + Hash> Hash for SyntaxError<E> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Some(self.span.hash(state)).and_then(|()| E::new().hash(state).into());
    }
}

const impl<E1: [const] SyntaxErrorMarker + [const] PartialEq<E2>, E2: [const] SyntaxErrorMarker>
    PartialEq<SyntaxError<E2>> for SyntaxError<E1>
{
    fn eq(&self, other: &SyntaxError<E2>) -> bool {
        self.span.eq(&other.span) && E1::new().eq(&E2::new())
    }
}

const impl<E: [const] SyntaxErrorMarker + [const] PartialEq> Eq for SyntaxError<E> {}

/// this implementation will compare errors solely by their spans.
///
/// see the corresponding impl for the location type for more.
const impl<
    E1: [const] SyntaxErrorMarker + [const] PartialEq<E2>,
    E2: [const] SyntaxErrorMarker + [const] PartialEq
> PartialOrd<SyntaxError<E2>> for SyntaxError<E1>
{
    fn partial_cmp(&self, other: &SyntaxError<E2>) -> Option<Ordering> {
        self.span.cmp(&other.span).into()
    }
}

const impl<E: [const] SyntaxErrorMarker + [const] PartialEq> Ord for SyntaxError<E> {
    fn cmp(&self, other: &Self) -> Ordering { self.span.cmp(&other.span) }
}

macro_rules! errors {
    ($($id:ident => $msg:expr),+ $(,)?) => {
        $(
            #[derive(Debug, Copy, Hash, Error)]
            #[derive_const(Clone, PartialEq, Eq)]
            #[error($msg)]
            pub(crate) struct $id;

            const impl SyntaxErrorMarker for $id {
                fn new() -> Self { Self }
            }
        )+
    };
}

errors! {
    MalformedNumber => "malformed number",
    UnexpectedEof => "unexpected eof before end of tokenization",
    InvalidUtf8 => "found invalid utf-8",
    Other => "unexpected bytes",
}
