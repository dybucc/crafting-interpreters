use std::error::Error;

use crate::{
    ErrorTrace, ToError,
    tokenizer::{GroupedSyntaxError, SyntaxError},
};

macro_rules! syntax_trace_impl {
    ($(
        $item:ident
    ),+) => {
        $(
            impl SyntaxTrace for $item {}
        )+
    };
}

pub(crate) trait SyntaxTrace: Error + ErrorTrace + ToError {}

syntax_trace_impl!(SyntaxError, GroupedSyntaxError);
