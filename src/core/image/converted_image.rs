use image::{ColorType, ImageBuffer, RgbImage, RgbaImage};

pub struct ConvertedImage {
    pub data: Vec<u8>,
    pub color_type: ColorType,
    pub width: u32,
    pub height: u32,
}

impl ConvertedImage {
    /// new
    pub fn new(data: &Vec<u8>, color_type: &ColorType, width: u32, height: u32) -> Self {
        Self {
            data: data.to_owned(),
            color_type: color_type.to_owned(),
            width,
            height,
        }
    }

    /// save as
    pub fn save_as(&self, file_path: &str) {
        if self.color_type == ColorType::Rgba8 {
            let output_image: RgbaImage =
                ImageBuffer::from_vec(self.width, self.height, self.data.clone()).unwrap();
            let _ = output_image.save(file_path);
        } else {
            let output_image: RgbImage =
                ImageBuffer::from_vec(self.width, self.height, self.data.clone()).unwrap();
            let _ = output_image.save(file_path);
        }
    }
}
