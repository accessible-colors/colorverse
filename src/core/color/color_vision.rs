use crate::core::color::color_basis::ColorBasis;

#[derive(Clone, Default, Debug)]
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
    /// inability to differentiate between green and red
    Protanopia,
    /// inability to differentiate between green and red
    Deuteranopia,
    /// inability to differentiate between blue and green, purple and red, and yellow and pink
    Tritanopia,
    /// absence of color discrimination
    Achromatopsia,
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
            ColorVision::Protanopia => ColorBasis::from(
                [0.56667, 0.43333, 0.0].into(),
                [0.55833, 0.44167, 0.0].into(),
                [0.0, 0.24167, 0.75833].into(),
            ),
            ColorVision::Deuteranopia => ColorBasis::from(
                [0.625, 0.375, 0.0].into(),
                [0.70, 0.30, 0.0].into(),
                [0.0, 0.30, 0.70].into(),
            ),
            ColorVision::Tritanopia => ColorBasis::from(
                [0.95, 0.5, 0.0].into(),
                [0.0, 0.43333, 0.56667].into(),
                [0.0, 0.475, 0.525].into(),
            ),
            ColorVision::Achromatopsia => ColorBasis::from(
                [0.299, 0.587, 0.114].into(),
                [0.299, 0.587, 0.114].into(),
                [0.299, 0.587, 0.114].into(),
            ),
        }
    }
}
