use argh::FromArgs;

use crate::cli::args::sub_command::SubCommand;

pub mod sub_command;

#[derive(FromArgs)]
#[argh(description = "args")]
pub struct Args {
    #[argh(subcommand)]
    pub sub_command: SubCommand,
}
