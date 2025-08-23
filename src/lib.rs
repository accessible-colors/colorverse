use crate::{
    color_vision::color_vision_type::ColorVisionType,
    core::image::{converted_image::ConvertedImage, source_image::SourceImage},
};

pub mod color_vision;
mod core;

/// convert for simulation
pub fn simulate(
    file_path: &str,
    color_vision_type: &ColorVisionType,
    simulation_level: f64,
) -> Result<ConvertedImage, String> {
    SourceImage::new(file_path).simulate(color_vision_type, simulation_level)
}

/// convert for daltonization
pub fn daltonize(
    file_path: &str,
    color_vision_type: &ColorVisionType,
    simulation_level: f64,
) -> Result<ConvertedImage, String> {
    SourceImage::new(file_path).daltonize(color_vision_type, simulation_level)
}

#[cfg(test)]
mod tests {
    use crate::color_vision::color_vision_type::color_vision_type_iterator::ColorVisionTypeIterator;

    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);

        // let color_vision = ColorVision::Protanopia;
        // let input_file_path = "tests/fixtures/input.png";
        // let output_file_path = format!("output-{}.png", color_vision);

        // convert(input_file_path, &color_vision_type, 0.5)
        //     .unwrap()
        //     .save_as(output_file_path.as_str());

        let mut color_vision_type_iterator =
            ColorVisionTypeIterator::new(&ColorVisionType::Trichromacy);
        while let Some(color_vision) = color_vision_type_iterator.next() {
            for simulation_level in [0.5, 1.0] {
                match simulate("tests/fixtures/input.png", &color_vision, simulation_level) {
                    Ok(x) => {
                        let output_file_path = format!(
                            "simulate-{}-{}.png",
                            &color_vision,
                            simulation_level * 100.0
                        );
                        x.save_as(output_file_path.as_str());
                    }
                    Err(err) => eprintln!("{}", err),
                }
                match daltonize("tests/fixtures/input.png", &color_vision, simulation_level) {
                    Ok(x) => {
                        let output_file_path = format!(
                            "daltonize-{}-{}.png",
                            &color_vision,
                            simulation_level * 100.0
                        );
                        x.save_as(output_file_path.as_str());
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
    }
}
