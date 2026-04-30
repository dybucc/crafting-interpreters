#![feature(bufreader_peek, string_from_utf8_lossy_owned)]

use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
    process::Termination,
};

use clap::Parser;

mod args;
mod error_rt;
#[macro_use]
mod macros;
mod runtime;
mod support;
mod tokenizer;

#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "In this project, errors are meant to be wildard-imported."
    )
)]
pub(crate) use crate::error_rt::*;
pub(crate) use crate::{args::Args, runtime::run};

fn main() -> anyhow::Result<()> {
    if let Args { script: Some(file) } = Args::parse() {
        run_file(&file)
    } else {
        run_prompt()
    }
}

pub(crate) fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> {
    run(&fs::read_to_string(file)?, &mut io::stdout().lock())
}

pub(crate) fn run_prompt() -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    let mut stderr = io::stderr().lock();

    let mut buf = String::new();

    loop {
        writef!(stdout, "> ")?;

        // NOTE: I already tried having the `into()` call be right outside the loop, but
        // that still means the expression that the loop evaluates to is of differing
        // types with `other_error`'s (later down the line.)
        match stdin.read_line(&mut buf) {
            Ok(0) => break writef!(stderr).map_err(Into::into),
            Err(err) => break Err(Into::into(err)),
            _ => (),
        }

        if let Err(err) = run(buf.trim_ascii_end(), &mut stdout) {
            match err.downcast::<crate::Error>() {
                Ok(lang_err) => {
                    writef!(stderr)?;
                    lang_err.into_result().report();
                    writef!(stderr)?;
                }
                Err(other_error) => break Err(other_error),
            }
        }

        buf.clear();
    }
}
