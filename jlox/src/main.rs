#![feature(unboxed_closures, tuple_trait, fn_traits)]

use std::{
  fs,
  io::{self, BufRead, StdoutLock, Write},
  iter,
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
  match Args::parse() {
    | Args { script: Some(file) } => run_file(Path::new(&file)),
    | _ => run_prompt(),
  }
}

pub(crate) fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> {
  Ok(run(&fs::read_to_string(file)?, &mut io::stdout().lock())?)
}

pub(crate) fn run_prompt() -> anyhow::Result<()> {
  iter::once((io::stdout().lock(), io::stdin().lock(), String::new()))
    .try_for_each(|(mut stdout, mut stdin, mut buf)| {
      iter::once(()).cycle().try_for_each(|()| {
        (
          write!(stdout, "> ")?,
          stdout.flush()?,
          match stdin.read_line(&mut buf)? {
            | 0 => Ok(()),
            | _ => (run(&buf, stdout.by_ref()), buf.clear()).0,
          },
        )
          .2
      })
    })
}

pub(crate) fn run(input: &str, stdout: &mut StdoutLock) -> anyhow::Result<()> {
  Ok(())
}
