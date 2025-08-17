use std::fmt;

use super::ColorBasis;

pub mod color_vision_iterator;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum ColorVision {
    // (normal) condition to see the full range of colors with three color cones
    #[default]
    Trichromacy,
    /// condition where red looks more green
    Protanomaly,
    /// condition where green looks more red
    Deuteranomaly,
    /// difficulty differentiating between blue and green, and between yellow and red
    Tritanomaly,
    /// all of three color cones have some form of deficiency
    Achromatomaly,
}

impl ColorVision {
    /// returns the percentages of colors to mix corresponding to each type of color vision
    ///
    /// source: https://web.archive.org/web/20081014161121/http://www.colorjack.com/labs/colormatrix/
    pub fn vec3(&self) -> ColorBasis {
        match self {
            ColorVision::Trichromacy => ColorBasis::new(),
            ColorVision::Protanomaly => ColorBasis::from(
                [0.81667, 0.18333, 0.0].into(),
                [0.33333, 0.66667, 0.0].into(),
                [0.0, 0.125, 0.875].into(),
            ),
            ColorVision::Deuteranomaly => ColorBasis::from(
                [0.80, 0.20, 0.0].into(),
                [0.25833, 0.74167, 0.0].into(),
                [0.0, 0.14167, 0.85833].into(),
            ),
            ColorVision::Tritanomaly => ColorBasis::from(
                [0.96667, 0.3333, 0.0].into(),
                [0.0, 0.73333, 0.26667].into(),
                [0.0, 0.18333, 0.81667].into(),
            ),
            ColorVision::Achromatomaly => ColorBasis::from(
                [0.618, 0.32, 0.62].into(),
                [0.163, 0.775, 0.62].into(),
                [0.163, 0.320, 0.516].into(),
            ),
        }
    }
}

impl fmt::Display for ColorVision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorVision::Trichromacy => write!(f, "trichromacy"),
            ColorVision::Protanomaly => write!(f, "protanomaly"),
            ColorVision::Deuteranomaly => write!(f, "deuteranomaly"),
            ColorVision::Tritanomaly => write!(f, "tritanomaly"),
            ColorVision::Achromatomaly => write!(f, "achromatomaly"),
        }
    }
}
