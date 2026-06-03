#![feature(
    bufreader_peek,
    string_from_utf8_lossy_owned,
    derive_const,
    const_trait_impl,
    const_cmp,
    const_default,
    const_clone
)]

use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
    process::Termination,
};

use clap::Parser;

mod args;
mod runtime;
mod support;
mod tokenizer;

use crate::{args::Args, runtime::run, tokenizer::TokenizerError};

fn main() -> anyhow::Result<()> {
    if let Some(file) = Args::parse().script() {
        run_file(file)
    } else {
        run_prompt()
    }
}

fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> {
    run(&fs::read_to_string(file)?, &mut io::stdout().lock())
}

fn run_prompt() -> anyhow::Result<()> {
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
            match err.downcast::<SyntaxError>() {
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

#[macro_use]
mod macros;

#[expect(
    clippy::single_component_path_imports,
    reason = "The macro is meant to be reexported at the crate level, and more specifically, at \
              the very end of the entry point to the binary, such that this reexport truly opens \
              up use to all crates as if it had been declared locally at the top of the module."
)]
pub(crate) use writef;
