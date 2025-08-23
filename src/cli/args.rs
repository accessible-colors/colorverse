use argh::FromArgs;
use colorverse::ColorVisionTypeForCli;

use crate::cli::args::mode::Mode;

pub mod mode;

#[derive(FromArgs)]
#[argh(description = "args")]
pub struct Args {
    /// input file path
    #[argh(positional)]
    pub file_path: String,

    /// output file path
    #[argh(option, short = 'o')]
    pub output_file: String,

    /// conversion mode
    #[argh(option, short = 'm')]
    pub mode: Mode,

    /// color vision simulation level
    #[argh(option, short = 'c')]
    pub color_vision: ColorVisionTypeForCli,

    /// color vision simulation level
    #[argh(option, short = 'l', default = "1.0")]
    pub simulation_level: f64,

    /// color vision simulation level
    #[argh(option, short = 's', default = "1.0")]
    pub daltonization_strength: f64,

    /// color vision simulation level
    #[argh(switch)]
    pub no_preserve_luminance: bool,
}
