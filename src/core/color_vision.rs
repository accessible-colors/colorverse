use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use nalgebra::{Matrix3, Vector3};

pub mod color_vision_type;

use crate::core::color_vision::color_vision_type::ColorVisionType;

/// RGB -> LMS conversion matrices (from Machado et al.)
const RGB_TO_LMS: [[f64; 3]; 3] = [
    [0.31399022, 0.63951294, 0.04649755],
    [0.15537241, 0.75789446, 0.08670142],
    [0.01775239, 0.10944209, 0.87256922],
];
/// RGB <- LMS conversion matrices (from Machado et al.)
const LMS_TO_RGB: [[f64; 3]; 3] = [
    [5.47221206, -4.6419601, 0.16963708],
    [-1.1252419, 2.29317094, -0.1678952],
    [0.02980165, -0.19318073, 1.16364789],
];

/// simulate color vision using a continuous parametric model inspired by Machado et al.
/// improved to apply a nonlinear threshold so that moderate levels (e.g., 0.5) have visible effects.
pub fn simulate_color_vision(
    img: &DynamicImage,
    color_vision_type: &ColorVisionType,
    level: f64,
) -> DynamicImage {
    let (width, height) = img.dimensions();
    // todo: performance tuning: image without alpha channel should be processed with rgbimage ?
    let mut output = RgbaImage::new(width, height);

    // matrix wrappers
    let m_rgb_to_lms = Matrix3::from_row_slice(&RGB_TO_LMS.concat());
    let m_lms_to_rgb = Matrix3::from_row_slice(&LMS_TO_RGB.concat());

    // nonlinear easing of level: more gradual at low level, faster at higher level
    let t = level.clamp(0.0, 1.0).powf(1.5);

    for (x, y, pixel) in img.pixels() {
        let rgba = pixel.0;
        let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);

        // convert sRGB -> linear [0,1]
        let srgb_to_linear = |u: u8| {
            let x = u as f64 / 255.0;
            if x <= 0.04045 {
                x / 12.92
            } else {
                ((x + 0.055) / 1.055).powf(2.4)
            }
        };
        let mut rgb_lin = Vector3::new(srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b));

        if let ColorVisionType::Trichromacy = color_vision_type {
            // normal vision: no change
            // todo: return Result instead ?
            return img.to_owned();
        } else if let ColorVisionType::Achromatomaly = color_vision_type {
            // grayscale conversion using nonlinear severity so moderate level has visible effect
            let lum = 0.2126 * rgb_lin.x + 0.7152 * rgb_lin.y + 0.0722 * rgb_lin.z;
            rgb_lin = rgb_lin * (1.0 - t) + Vector3::new(lum, lum, lum) * t;
        } else {
            // convert to LMS space
            let mut lms = m_rgb_to_lms * rgb_lin;

            // apply anomaly transform with nonlinear severity
            match color_vision_type {
                ColorVisionType::Protanomaly => {
                    let l_new = (1.0 - t) * lms.x + t * (0.68 * lms.y + 0.32 * lms.z);
                    lms.x = l_new;
                }
                ColorVisionType::Deuteranomaly => {
                    let m_new = (1.0 - t) * lms.y + t * (0.70 * lms.x + 0.30 * lms.z);
                    lms.y = m_new;
                }
                ColorVisionType::Tritanomaly => {
                    let s_new = (1.0 - t) * lms.z + t * (0.65 * lms.x + 0.35 * lms.y);
                    lms.z = s_new;
                }
                _ => {}
            }

            // convert back to linear RGB
            rgb_lin = m_lms_to_rgb * lms;
        }

        // convert linear -> sRGB
        let linear_to_srgb = |x: f64| {
            let x = if x <= 0.0031308 {
                12.92 * x
            } else {
                1.055 * x.powf(1.0 / 2.4) - 0.055
            };
            (x * 255.0).round().clamp(0.0, 255.0) as u8
        };

        let r_out = linear_to_srgb(rgb_lin.x);
        let g_out = linear_to_srgb(rgb_lin.y);
        let b_out = linear_to_srgb(rgb_lin.z);

        output.put_pixel(x, y, Rgba([r_out, g_out, b_out, a]));
    }

    DynamicImage::from(output)
}
