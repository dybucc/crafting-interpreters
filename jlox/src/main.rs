#![feature(unboxed_closures, tuple_trait, fn_traits)]

use std::{
  convert::Infallible,
  fs::{self, File},
  io::{self, BufReader, Read},
  marker::Tuple,
  path::Path,
  process,
};

use anyhow::Ok;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(disable_version_flag = true, disable_help_subcommand = true)]
pub(crate) struct Args {
  script: Option<String>,
}

#[derive(Debug)]
pub(crate) enum Error {
  Usage,
  DataErr,
  NoInput,
  NoUser,
  NoHost,
  Unavailable,
  Software,
  OsErr,
  OsFile,
  CantCreat,
  IoErr,
  TempFail,
  Protocol,
  NoPerm,
  Config,
}

impl Error {
  pub(crate) fn into_result(self) -> anyhow::Result<Self> { Ok(self) }
}

impl<A: Tuple> FnOnce<A> for Error {
  type Output = Infallible;

  extern "rust-call" fn call_once(self, _: A) -> Self::Output {
    match self {
      | Self::Usage => process::exit(64),
      | Self::DataErr => process::exit(65),
      | Self::NoInput => process::exit(66),
      | Self::NoUser => process::exit(67),
      | Self::NoHost => process::exit(68),
      | Self::Unavailable => process::exit(69),
      | Self::Software => process::exit(70),
      | Self::OsErr => process::exit(71),
      | Self::OsFile => process::exit(72),
      | Self::CantCreat => process::exit(73),
      | Self::IoErr => process::exit(74),
      | Self::TempFail => process::exit(75),
      | Self::Protocol => process::exit(76),
      | Self::NoPerm => process::exit(77),
      | Self::Config => process::exit(78),
    }
  }
}

fn main() -> anyhow::Result<()> {
  match Args::parse() {
    | Args { script: Some(file) } => run_file(Path::new(&file)),
    | _ => run(io::stdin()),
  }
}

pub(crate) fn run_file(file: impl AsRef<Path>) -> anyhow::Result<()> {
  run(File::open(file)?)
}

pub(crate) fn run(input: impl Read) -> anyhow::Result<()> {
  let reader = BufReader::new(input);

  Ok(())
}
