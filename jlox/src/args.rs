use clap::Parser;

#[derive(Debug, Parser)]
#[command(disable_version_flag = true, disable_help_subcommand = true)]
pub(crate) struct Args {
  pub(crate) script: Option<String>,
}
