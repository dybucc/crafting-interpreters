//! This module holds the basic error interface of all language errors. The
//! mechanism is fairly strightforward, as each error is meant to implement one
//! or two of two traits; Namely, `ErrorTrace` and/or `ToError`.
//!
//! The former provides the backtrace of the error, which by default returns the
//! `Display` representation of the error. The latter is optional, as it allows
//! customizing the implementation of the conversion to `Error` by providing an
//! additional error message, if it has one. This is shown in the `Caused by`
//! section of `anyhow::Error` (which itself gets called from `Result`'s impl of
//! `Termination`.)

use std::{borrow::Cow, error::Error as StdError, fmt::Debug};

use anyhow::{Context, anyhow};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{}", .trace.build())]
pub(crate) struct Error {
    pub(crate) trace: Box<dyn ErrorTrace>,
    pub(crate) msg: Option<Cow<'static, str>>,
}

pub(crate) trait ToError: ErrorTrace
where
    for<'a> Self: Sized + 'a,
{
    fn convert(self, msg: Option<Cow<'static, str>>) -> Error {
        Error {
            trace: Box::new(self) as Box<dyn ErrorTrace>,
            msg,
        }
    }
}

impl Error {
    pub(crate) fn into_result(self) -> anyhow::Result<()> {
        let Self { trace, msg } = self;
        let err = anyhow!(trace.build());

        if let Some(msg) = msg {
            Err(err).context(msg)
        } else {
            Err(err)
        }
    }
}

pub(crate) trait ErrorTrace: StdError + Send + Sync {
    fn build(&self) -> Cow<'static, str> {
        format!("{self}").into()
    }
}
