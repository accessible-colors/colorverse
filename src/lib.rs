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
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        // let _ = convert(ColorVision::Protanopia);
        let _ = convert("input.png", &ColorVision::Deuteranopia)
            .unwrap()
            .save_as("output.png");
    }
}
