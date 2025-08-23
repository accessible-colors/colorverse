pub mod cli;

use cli::args::Args;
use colorverse::{daltonize, simulate};

use crate::cli::args::sub_command::SubCommand;

fn main() {
    let args: Args = argh::from_env();

    let (converted_image, output_file) = match args.sub_command {
        SubCommand::Simulate(args) => {
            println!("simulation starts");
            (
                simulate(
                    args.file_path.as_str(),
                    &args.color_vision,
                    args.simulation_level,
                ),
                args.output_file,
            )
        }
        SubCommand::Daltonize(args) => {
            println!("daltonization starts");
            (
                daltonize(
                    args.file_path.as_str(),
                    &args.color_vision,
                    args.simulation_level,
                    args.daltonization_strength,
                    !args.no_preserve_luminance,
                ),
                args.output_file,
            )
        }
    };

    if let Ok(converted_image) = converted_image {
        converted_image.save_as(output_file.as_str());
    } else {
        eprint!("failed to convert")
    }
}
