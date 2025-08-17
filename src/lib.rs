use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use nalgebra::{Matrix3, Vector3};

use crate::core::{
    color_vision::color_vision_type::ColorVisionType,
    image::{converted_image::ConvertedImage, source_image::SourceImage},
};

mod core;

/// convert
pub fn convert(
    file_path: &str,
    color_vision_type: &ColorVisionType,
    level: f64,
) -> Result<ConvertedImage, String> {
    SourceImage::new(file_path).convert(color_vision_type, level)
}

#[cfg(test)]
mod tests {
    use crate::{
        convert,
        core::color_vision::color_vision_type::color_vision_type_iterator::ColorVisionTypeIterator,
    };

    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);

        // let color_vision = ColorVision::Protanopia;
        // let input_file_path = "input.png";
        // let output_file_path = format!("output-{}.png", color_vision);

        // convert(input_file_path, &color_vision_type, 0.5)
        //     .unwrap()
        //     .save_as(output_file_path.as_str());

        let mut color_vision_type_iterator =
            ColorVisionTypeIterator::new(&ColorVisionType::Trichromacy);
        while let Some(color_vision) = color_vision_type_iterator.next() {
            for level in [0.5, 1.0] {
                match convert("input.png", &color_vision, level) {
                    Ok(x) => {
                        let output_file_path =
                            format!("output-{}-{}.png", &color_vision, level * 100.0);
                        x.save_as(output_file_path.as_str());
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
    }
}
