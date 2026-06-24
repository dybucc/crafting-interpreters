#![feature(
    bufreader_peek, string_from_utf8_lossy_owned, derive_const, const_trait_impl, const_cmp,
    const_default, const_clone, const_convert, bstr
)]

use std::path::Path;

mod args;
mod runtime;
mod support;
mod tokenizer;

fn main() -> anyhow::Result<()> { todo!() }

fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> { todo!() }

fn run_prompt() -> anyhow::Result<()> { todo!() }

#[macro_use]
mod macros;
