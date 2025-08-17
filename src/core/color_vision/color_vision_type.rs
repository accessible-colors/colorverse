use std::fmt;

pub mod color_vision_type_iterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorVisionType {
    // (normal) condition to see the full range of colors with three color cones
    Trichromacy,
    /// l cone deficiency: condition where red looks more green
    Protanomaly,
    /// m cone deficiency: condition where green looks more red
    Deuteranomaly,
    /// s cone deficiency: difficulty differentiating between blue and green, and between yellow and red
    Tritanomaly,
    /// all of three color cones have some form of deficiency
    Achromatomaly,
}

impl fmt::Display for ColorVisionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorVisionType::Trichromacy => write!(f, "trichromacy"),
            ColorVisionType::Protanomaly => write!(f, "protanomaly"),
            ColorVisionType::Deuteranomaly => write!(f, "deuteranomaly"),
            ColorVisionType::Tritanomaly => write!(f, "tritanomaly"),
            ColorVisionType::Achromatomaly => write!(f, "achromatomaly"),
        }
    }
}
