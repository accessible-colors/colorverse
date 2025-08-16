use crate::core::{color::color_basis::ColorBasis, vec3::Vec3};

/// transform color to specific basis
pub fn transform_color(basis: &ColorBasis, original_rgb: &Vec3) -> Vec3 {
    // multiply the base vector and the original RGB values for each channel
    let new_red = basis.red.scale(original_rgb.x);
    let new_green = basis.green.scale(original_rgb.y);
    let new_blue = basis.blue.scale(original_rgb.z);
    // sum up the result
    new_red.add(&new_green).add(&new_blue)
}
