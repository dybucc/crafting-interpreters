use clap::Parser;

#[derive(Debug, Parser)]
#[command(disable_version_flag = true, disable_help_subcommand = true)]
pub(crate) struct Args {
    script: Option<String>,
}

impl Args {
    pub(crate) fn script(&self) -> Option<&String> {
        self.script.as_ref()
    }
}
