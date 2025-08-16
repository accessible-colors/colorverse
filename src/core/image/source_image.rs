use image::{ColorType, RgbImage, RgbaImage};

use crate::core::color::{color_vision::ColorVision, convert_image_color};

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
    pub fn convert(&self, color_vision: &ColorVision) -> Result<ConvertedImage, String> {
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

        // basis vector
        let color_basis = color_vision.vec3();

        // convert image and get it as vec
        let converted_image_data = if color_type == ColorType::Rgba8 {
            // todo: debug log
            println!("image has an alpha channel for the transparency information to be preserved");

            let mut img: RgbaImage = dynamic_image.to_rgba8();
            convert_image_color(&mut img, &color_basis);
            img.into_vec()
        } else {
            // todo: debug log
            println!("image doesn't have an alpha channel so will be processed as RGB");

            let mut img: RgbImage = dynamic_image.to_rgb8();
            convert_image_color(&mut img, &color_basis);
            img.into_vec()
        };

        Ok(ConvertedImage::new(
            &converted_image_data,
            &color_type,
            dynamic_image.width(),
            dynamic_image.height(),
        ))
    }
}
