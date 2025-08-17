use image::ColorType;

use crate::core::color_vision::{color_vision_type::ColorVisionType, simulate_color_vision};

use super::converted_image::ConvertedImage;

pub struct SourceImage {
    pub file_path: String,
}

impl SourceImage {
    /// new
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_owned(),
        }
    }

    /// convert
    pub fn convert(
        &self,
        color_vision_type: &ColorVisionType,
        level: f64,
    ) -> Result<ConvertedImage, String> {
        // todo: debug log
        println!("load image: {}", self.file_path);

        let dynamic_image = match image::open(self.file_path.as_str()) {
            Ok(dynamic_image) => dynamic_image,
            Err(e) => {
                eprintln!("failed to load image: {}", e);
                return Err("todo".to_owned());
            }
        };

        // convert to the appropriate type based on rgb(a) format
        let color_type = if dynamic_image.color().has_alpha() {
            ColorType::Rgba8
        } else {
            ColorType::Rgb8
        };

        let converted_image = simulate_color_vision(&dynamic_image, color_vision_type, level);

        Ok(ConvertedImage::new(
            converted_image.into_bytes().as_ref(),
            &color_type,
            dynamic_image.width(),
            dynamic_image.height(),
        ))
    }
}
