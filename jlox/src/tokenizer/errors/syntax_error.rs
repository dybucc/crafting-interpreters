use std::{error::Error, marker::PhantomData};

use super::super::Location;

mod id;
mod types;

pub(crate) use self::types::*;

pub(crate) const trait SyntaxErrorMarker: Error {
    fn new() -> Self;
}

#[derive(Debug)]
pub(crate) struct SyntaxError<E: SyntaxErrorMarker> {
    span: Location,
    _marker: PhantomData<E>
}

pub(crate) fn new<E: SyntaxErrorMarker>(l: Location) -> SyntaxError<E> { todo!() }
