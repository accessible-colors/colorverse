use nalgebra::Matrix3;

use super::color_matrices::{LMS_TO_RGB, RGB_TO_LMS};

/// convert sRGB (0..255 u8) to linear RGB (0.0..1.0)
pub fn srgb8_to_linear(v: u8) -> f64 {
    let x = (v as f64) / 255.0;
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

/// convert linear RGB (0.0..1.0) to sRGB u8 (0..255)
pub fn linear_to_srgb8(x: f64) -> u8 {
    let x = if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    };
    (x * 255.0).round().clamp(0.0, 255.0) as u8
}

/// wrappers to construct nalgebra matrices from const arrays
pub fn rgb_to_lms_matrix() -> Matrix3<f64> {
    Matrix3::from_row_slice(&RGB_TO_LMS.concat())
}
pub fn lms_to_rgb_matrix() -> Matrix3<f64> {
    Matrix3::from_row_slice(&LMS_TO_RGB.concat())
}

// todo: possibly useful as test utils ?
#[allow(dead_code)]
mod test {
    use nalgebra::Vector3;

    use crate::core::color_vision::color_utils::{lms_to_rgb_matrix, rgb_to_lms_matrix};

    /// convert linear RGB Vector3 (r,g,b) into LMS
    pub fn rgb_linear_to_lms(rgb: Vector3<f64>) -> Vector3<f64> {
        &rgb_to_lms_matrix() * rgb
    }
    /// Convert LMS into linear RGB Vector3
    pub fn lms_to_rgb_linear(lms: Vector3<f64>) -> Vector3<f64> {
        &lms_to_rgb_matrix() * lms
    }
}
