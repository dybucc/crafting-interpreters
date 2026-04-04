use std::{borrow::Cow, convert::Infallible, marker::Tuple, process};

use anyhow::{Context, anyhow};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum SysExitsError {
  #[error("command used incorrectly")]
  Usage,
  #[error("input data was incorrect")]
  DataErr,
  #[error("input file did not exist/was not readable")]
  NoInput,
  #[error("specified user did not exist")]
  NoUser,
  #[error("specified host did not exist")]
  NoHost,
  #[error("service unavailable")]
  Unavailable,
  #[error("internal software error")]
  Software,
  #[error("operating system error")]
  OsErr,
  #[error("system file error (can't create/read)")]
  OsFile,
  #[error("user output file can't be created")]
  CantCreat,
  #[error("io error while handling file")]
  IoErr,
  #[error("temporary failure; retry")]
  TempFail,
  #[error("remote system returned invalid protocol message")]
  Protocol,
  #[error("not enough permissions to perform operation")]
  NoPerm,
  #[error("uncofigured or misconfigured state found")]
  Config,
}

impl<A: Tuple> FnOnce<A> for SysExitsError {
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

#[derive(Debug)]
pub(crate) struct Error {
  line: usize,
  msg:  Cow<'static, str>,
}

impl<A: Tuple> FnOnce<A> for Error {
  type Output = anyhow::Result<()>;

  extern "rust-call" fn call_once(self, _: A) -> Self::Output {
    Result::Err(anyhow!("line: {}", self.line)).context(self.msg)
  }
}
