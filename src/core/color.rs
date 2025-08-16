use crate::core::vec3::Vec3;
use image::{ImageBuffer, Pixel};
use {color_basis::ColorBasis, util::transform_color};

mod color_basis;
pub mod color_vision;
mod util;

/// convert color in image
pub fn convert_image_color<P, Container>(
    img: &mut ImageBuffer<P, Container>,
    color_basis: &ColorBasis,
) where
    P: Pixel<Subpixel = u8> + 'static,
    Container: std::ops::Deref<Target = [P::Subpixel]> + std::ops::DerefMut,
{
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel_mut(x, y);

            let original_rgb = Vec3::from([
                pixel.channels()[0] as f32 / 255.0,
                pixel.channels()[1] as f32 / 255.0,
                pixel.channels()[2] as f32 / 255.0,
            ]);

            let new_rgb = transform_color(color_basis, &original_rgb);

            let out_channels = pixel.channels_mut();
            out_channels[0] = (new_rgb.x * 255.0).clamp(0.0, 255.0) as u8;
            out_channels[1] = (new_rgb.y * 255.0).clamp(0.0, 255.0) as u8;
            out_channels[2] = (new_rgb.z * 255.0).clamp(0.0, 255.0) as u8;
        }
    }
}
