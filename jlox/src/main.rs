#![feature(unboxed_closures, tuple_trait, fn_traits)]

use std::{
    fs,
    io::{self, BufRead, StdoutLock, Write},
    path::Path,
    process::Termination,
};

use clap::Parser;

pub(crate) mod args;
pub(crate) mod errors;

pub(crate) use crate::args::Args;
#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "Errors are meant to be wildard-imported."
    )
)]
pub(crate) use crate::errors::*;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let Args { script } = args;
    if let Some(file) = script {
        run_file(&file)
    } else {
        run_prompt()
    }
}

pub(crate) fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = {
        let res = fs::read_to_string(file);
        res?
    };
    let mut stdout = {
        let stdout = io::stdout();
        stdout.lock()
    };
    run(&file, &mut stdout)
}

pub(crate) fn run_prompt() -> anyhow::Result<()> {
    let mut stdout = {
        let stdout = io::stdout();
        stdout.lock()
    };
    let mut stdin = {
        let stdin = io::stdin();
        stdin.lock()
    };
    let mut buf = String::new();
    loop {
        let res = write!(stdout, ">");
        res?;
        let res = stdout.flush();
        res?;
        let res = stdin.read_line(&mut buf);
        res?;
        let res = run(&buf, &mut stdout);
        if let Err(err) = res {
            // If the error is a recognized error (i.e. `crate::errors::Error`,) then report
            // it and keep going. This only happens when running interactively,
            // so if there's a failure while running a script loaded from a
            // file, the whole thing just bails out.
            let try_downcast = err.downcast::<Error>();
            if let Ok(lang_err) = try_downcast {
                let res = lang_err();
                res.report();
            }
        }
    }
}

pub(crate) fn run(input: &str, stdout: &mut StdoutLock) -> anyhow::Result<()> {
    anyhow::Ok(())
}
