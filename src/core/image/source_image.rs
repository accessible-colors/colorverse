use image::{ColorType, DynamicImage};

use crate::color_vision::{
    color_vision_type::ColorVisionType, daltonize_color_vision, simulate_color_vision,
};

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

    /// simulate
    pub fn simulate(
        &self,
        color_vision_type: &ColorVisionType,
        simulation_level: f64,
    ) -> Result<ConvertedImage, String> {
        let (dynamic_image, color_type) =
            match dynamic_image_and_color_type(self.file_path.as_str()) {
                Ok(x) => (x.0, x.1),
                Err(err) => panic!("{}", err),
            };

        let converted_image =
            simulate_color_vision(&dynamic_image, color_vision_type, simulation_level);

        Ok(ConvertedImage::new(
            converted_image.into_bytes().as_ref(),
            &color_type,
            dynamic_image.width(),
            dynamic_image.height(),
        ))
    }

    /// daltonize
    pub fn daltonize(
        &self,
        color_vision_type: &ColorVisionType,
        simulation_level: f64,
    ) -> Result<ConvertedImage, String> {
        let (dynamic_image, color_type) =
            match dynamic_image_and_color_type(self.file_path.as_str()) {
                Ok(x) => (x.0, x.1),
                Err(err) => panic!("{}", err),
            };

        // todo: strength arg
        let converted_image = daltonize_color_vision(
            &dynamic_image,
            color_vision_type,
            simulation_level,
            1.0,
            true,
        );

        Ok(ConvertedImage::new(
            converted_image.into_bytes().as_ref(),
            &color_type,
            dynamic_image.width(),
            dynamic_image.height(),
        ))
    }
}

fn dynamic_image_and_color_type(file_path: &str) -> Result<(DynamicImage, ColorType), String> {
    // todo: debug log
    println!("load image: {}", file_path);

    let dynamic_image = match image::open(file_path) {
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

    Ok((dynamic_image, color_type))
}
