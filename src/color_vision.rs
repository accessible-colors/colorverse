use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use nalgebra::{Matrix3, Vector3};

mod color_matrices;
mod color_utils;
pub mod color_vision_type;

use crate::color_vision::color_vision_type::ColorVisionType;
use color_matrices::{LMS_TO_RGB, RGB_TO_LMS};

/// simulate color vision using a continuous parametric model inspired by Machado et al.
/// improved to apply a nonlinear threshold so that moderate simulation_levels (e.g., 0.5) have visible effects.
pub fn simulate_color_vision(
    img: &DynamicImage,
    color_vision_type: &ColorVisionType,
    simulation_level: f64,
) -> DynamicImage {
    let (width, height) = img.dimensions();
    // todo: performance tuning: image without alpha channel should be processed with rgbimage ?
    let mut output = RgbaImage::new(width, height);

    // matrix wrappers
    let m_rgb_to_lms = Matrix3::from_row_slice(&RGB_TO_LMS.concat());
    let m_lms_to_rgb = Matrix3::from_row_slice(&LMS_TO_RGB.concat());

    // nonlinear easing of simulation_level: more gradual at low simulation_level, faster at higher simulation_level
    let t = simulation_level.clamp(0.0, 1.0).powf(1.5);

    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            let a = p[3];

            // sRGB u8 -> linear RGB (0..1)
            let mut rgb_lin = Vector3::new(
                {
                    let x = (p[0] as f64) / 255.0;
                    if x <= 0.04045 {
                        x / 12.92
                    } else {
                        ((x + 0.055) / 1.055).powf(2.4)
                    }
                },
                {
                    let x = (p[1] as f64) / 255.0;
                    if x <= 0.04045 {
                        x / 12.92
                    } else {
                        ((x + 0.055) / 1.055).powf(2.4)
                    }
                },
                {
                    let x = (p[2] as f64) / 255.0;
                    if x <= 0.04045 {
                        x / 12.92
                    } else {
                        ((x + 0.055) / 1.055).powf(2.4)
                    }
                },
            );

            // when achromatomaly (near grayscale), transform differently:
            if let ColorVisionType::Achromatomaly = color_vision_type {
                // mix toward luminance by t
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
                    x * 12.92
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
    }

    DynamicImage::from(output)
}

/// daltonize (color blindness correction) based on simulation difference and redistribution.
pub fn daltonize_color_vision(
    img: &DynamicImage,
    color_vision_type: &ColorVisionType,
    simulation_level: f64,
    daltonization_strength: f64,
    preserve_luminance: bool,
) -> DynamicImage {
    // 1) obtain simulated image using existing function (unaltered)
    let sim_img = simulate_color_vision(img, color_vision_type, simulation_level);

    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    // matrices
    let m_rgb_to_lms = color_utils::rgb_to_lms_matrix();
    let m_lms_to_rgb = color_utils::lms_to_rgb_matrix();

    // luminance vector
    let y_vec = nalgebra::Vector3::new(0.2126, 0.7152, 0.0722);

    for y in 0..height {
        for x in 0..width {
            let p_orig = img.get_pixel(x, y);
            let p_sim = sim_img.get_pixel(x, y);
            let a = p_orig[3];

            // convert original and sim to linear rgb (Vector3)
            let orig_lin = nalgebra::Vector3::new(
                color_utils::srgb8_to_linear(p_orig[0]),
                color_utils::srgb8_to_linear(p_orig[1]),
                color_utils::srgb8_to_linear(p_orig[2]),
            );
            let sim_lin = nalgebra::Vector3::new(
                color_utils::srgb8_to_linear(p_sim[0]),
                color_utils::srgb8_to_linear(p_sim[1]),
                color_utils::srgb8_to_linear(p_sim[2]),
            );

            // to LMS
            let orig_lms = m_rgb_to_lms * orig_lin;
            let sim_lms = m_rgb_to_lms * sim_lin;

            // difference (lost info)
            let diff = orig_lms - sim_lms;

            // redistribution vector to add back into sim_lms
            let mut add = nalgebra::Vector3::zeros();

            match color_vision_type {
                ColorVisionType::Protanomaly => {
                    // redistribute L error into M and S
                    let e_l = diff.x;
                    add.y += e_l * 0.6;
                    add.z += e_l * 0.4;
                }
                ColorVisionType::Deuteranomaly => {
                    let e_m = diff.y;
                    add.x += e_m * 0.7;
                    add.z += e_m * 0.3;
                }
                ColorVisionType::Tritanomaly => {
                    let e_s = diff.z;
                    add.x += e_s * 0.5;
                    add.y += e_s * 0.5;
                }
                ColorVisionType::Achromatomaly => {
                    // slightly enhance L/M balance for achromatomaly
                    let avg = (diff.x + diff.y) * 0.5;
                    add.x += avg * 0.5;
                    add.y += avg * 0.5;
                    // keep S small
                    add.z += diff.z * 0.2;
                }
                _ => {}
            }

            // apply daltonization strength scaling
            add *= daltonization_strength.clamp(0.0, 1.0);

            // corrected LMS = sim_lms + add
            let corrected_lms = sim_lms + add;

            // convert back to linear RGB
            let mut corrected_rgb = m_lms_to_rgb * corrected_lms;

            // optional: preserve luminance by removing Y component from the rgb delta
            if preserve_luminance {
                let delta_rgb = corrected_rgb - sim_lin;
                let y_change = y_vec.dot(&delta_rgb);
                // projection of delta_rgb onto y_vec
                let y_vec_norm_sq = y_vec.dot(&y_vec);
                if y_vec_norm_sq > 0.0 {
                    let proj = y_vec * (y_change / y_vec_norm_sq);
                    corrected_rgb = sim_lin + (delta_rgb - proj);
                }
            }

            // guard NaN/Inf and clamp
            for c in 0..3 {
                if !corrected_rgb[c].is_finite() {
                    corrected_rgb[c] = sim_lin[c];
                }
                if corrected_rgb[c].is_nan() {
                    corrected_rgb[c] = 0.0;
                }
                // clamp to reasonable range before converting
                corrected_rgb[c] = corrected_rgb[c].clamp(0.0, 1.0);
            }

            let r_out = color_utils::linear_to_srgb8(corrected_rgb.x);
            let g_out = color_utils::linear_to_srgb8(corrected_rgb.y);
            let b_out = color_utils::linear_to_srgb8(corrected_rgb.z);

            output.put_pixel(x, y, Rgba([r_out, g_out, b_out, a]));
        }
    }

    DynamicImage::from(output)
}
