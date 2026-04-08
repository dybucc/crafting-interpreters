#![feature(unboxed_closures, tuple_trait, fn_traits)]

use std::{
    fs,
    io::{self, BufRead, StdoutLock, Write},
    path::Path,
};

use anyhow::Ok;
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
    if let Some(file) = args.script {
        run_file(&file)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

pub(crate) fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = fs::read_to_string(file)?;
    let mut stdout = io::stdout().lock();
    run(&file, &mut stdout)?;
    Ok(())
}

pub(crate) fn run_prompt() -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    let mut buf = String::new();
    loop {
        write!(stdout, ">")?;
        stdout.flush()?;
        stdin.read_line(&mut buf)?;
    }
}

pub(crate) fn run(input: &str, stdout: &mut StdoutLock) -> anyhow::Result<()> {
    Ok(())
}
