use argh::FromArgs;
use colorverse::ColorVisionTypeForCli;

#[derive(FromArgs, PartialEq, Debug)]
/// simulate subcommand
#[argh(subcommand, name = "simulate")]
pub struct SimulateArgs {
    /// input file path
    #[argh(positional)]
    pub file_path: String,

    /// output file path
    #[argh(option, short = 'o')]
    pub output_file: String,

    /// color vision simulation level
    #[argh(option, short = 'c')]
    pub color_vision: ColorVisionTypeForCli,

    /// color vision simulation level
    #[argh(option, short = 'l', default = "1.0")]
    pub simulation_level: f64,
}
