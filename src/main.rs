use argh::FromArgs;
use colorverse::{color_vision::color_vision_type::ColorVisionType, simulate};

#[derive(FromArgs)]
#[argh(description = "args")]
struct Args {
    /// input file path
    #[argh(positional)]
    file_path: String,

    /// color vision simulation level
    #[argh(option, short = 'c')]
    color_vision: ColorVisionType,

    /// color vision simulation level
    #[argh(option, short = 'l', default = "1.0")]
    simulation_level: f64,

    /// output file path
    #[argh(option, short = 'o')]
    output_file: String,
}
fn main() {
    let args: Args = argh::from_env();
    // todo: simulate and daltonize
    if let Ok(converted_image) = simulate(
        args.file_path.as_str(),
        &args.color_vision,
        args.simulation_level,
    ) {
        converted_image.save_as(args.output_file.as_str());
    } else {
        eprint!("failed to convert")
    }
}
