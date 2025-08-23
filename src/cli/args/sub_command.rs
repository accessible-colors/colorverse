use argh::FromArgs;

use crate::cli::args::sub_command::{daltonize_args::DaltonizeArgs, simulate_args::SimulateArgs};

pub mod daltonize_args;
pub mod simulate_args;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Simulate(SimulateArgs),
    Daltonize(DaltonizeArgs),
}
