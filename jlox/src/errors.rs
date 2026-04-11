use std::{
    borrow::Cow,
    fmt::{Debug, Display},
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
    pub(crate) trace: Box<dyn ErrorTrace>,
    pub(crate) msg: Cow<'static, str>,
}

impl Error {
    pub(crate) fn new(trace: Box<dyn ErrorTrace>, msg: Option<Cow<'static, str>>) -> Self {
        let msg = {
            let into_map = || "".into();
            msg.unwrap_or_else(into_map)
        };
        Self { trace, msg }
    }

    pub(crate) fn into_result(self) -> anyhow::Result<()> {
        // The result of this would be an error where the main source of truth would be
        // the error backtrace, left to the implementor's discretion, and a further
        // source of truth (in the `Caused by` section of the error) containing some
        // arbitrary error message.
        let Self { trace, msg } = self;
        let err = anyhow!(msg);
        let trace = trace.build();
        let res = Err(err);
        res.context(trace)
    }
}

pub(crate) trait ErrorTrace: Display + Debug + Send + Sync {
    fn build(&self) -> Cow<'static, str> {
        let msg = format!("{self}");
        msg.into()
    }
}
