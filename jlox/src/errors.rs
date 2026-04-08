use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::Tuple,
};

use anyhow::{Context, anyhow};
use thiserror::Error;

// NOTE: the `Display` impl only reflects the immediate message returned by the
// error, and not the backtrace. This is intentional, as only calling in to the
// function that the `Error` type implements actually yields a `Result` with
// context provided by the backtrace.
#[derive(Debug, Error)]
#[error("{msg}")]
pub(crate) struct Error {
    trace: Box<dyn ErrorTrace>,
    msg: Cow<'static, str>,
}

impl Error {
    pub(crate) fn new(trace: Box<dyn ErrorTrace>, msg: Option<Cow<'static, str>>) -> Self {
        let msg = msg.unwrap_or_else(|| "".into());
        Self { trace, msg }
    }
}

impl<A: Tuple> FnOnce<A> for Error {
    type Output = anyhow::Result<()>;

    extern "rust-call" fn call_once(self, _: A) -> Self::Output {
        // The result of this would be an error where the main source of truth would
        // be the error backtrace, left to the implementor's discretion, and a
        // further source of truth (in the `Caused by` section of the error)
        // containing some other error message.
        Result::Err(anyhow!(self.msg)).context(self.trace.build())
    }
}

pub(crate) trait ErrorTrace: Display + Debug + Send + Sync {
    fn build(&self) -> Cow<'static, str> {
        format!("{self}").into()
    }
}
