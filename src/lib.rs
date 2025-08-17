use crate::core::{
    color::color_vision::ColorVision,
    image::{converted_image::ConvertedImage, source_image::SourceImage},
};

mod core;

/// convert
pub fn convert(file_path: &str, color_vision: &ColorVision) -> Result<ConvertedImage, String> {
    SourceImage::new(file_path).convert(color_vision)
}

#[cfg(test)]
mod tests {
    use crate::convert;
    use crate::core::color::color_vision::color_vision_iterator::ColorVisionIterator;

    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);

        let color_vision = ColorVision::Protanopia;
        let input_file_path = "input.png";
        let output_file_path = format!("output-{}.png", color_vision);

        convert(input_file_path, &color_vision)
            .unwrap()
            .save_as(output_file_path.as_str());

        let mut color_vision_iterator = ColorVisionIterator::new(&ColorVision::Trichromacy);
        while let Some(color_vision) = color_vision_iterator.next() {
            match convert("input.png", &color_vision) {
                Ok(x) => {
                    let output_file_path = format!("output-{}.png", &color_vision);
                    x.save_as(output_file_path.as_str());
                }
                Err(err) => eprintln!("{}", err),
            }
        }
    }
}
