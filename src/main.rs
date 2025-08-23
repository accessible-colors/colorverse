use cli::args::Args;
use colorverse::{daltonize, simulate};

use crate::cli::args::mode::Mode;

mod cli;

fn main() {
    let args: Args = argh::from_env();

    let converted_image = match args.mode {
        Mode::Simulate => {
            println!("simluation starts");
            simulate(
                args.file_path.as_str(),
                &args.color_vision,
                args.simulation_level,
            )
        }
        Mode::Daltonize => {
            println!("daltonization starts");
            daltonize(
                args.file_path.as_str(),
                &args.color_vision,
                args.simulation_level,
                args.daltonization_strength,
                !args.no_preserve_luminance,
            )
        }
    };

    if let Ok(converted_image) = converted_image {
        converted_image.save_as(args.output_file.as_str());
    } else {
        eprint!("failed to convert")
    }
}
