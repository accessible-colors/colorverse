use std::{fmt, str::FromStr};

use super::color_vision_type::color_vision_type_iterator::ColorVisionTypeIterator;

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

impl FromStr for ColorVisionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trichromacy" => Ok(ColorVisionType::Trichromacy),
            "protanomaly" => Ok(ColorVisionType::Protanomaly),
            "deuteranomaly" => Ok(ColorVisionType::Deuteranomaly),
            "tritanomaly" => Ok(ColorVisionType::Tritanomaly),
            "achromatomaly" => Ok(ColorVisionType::Achromatomaly),
            _ => {
                let mut color_vision_types = vec![ColorVisionType::Trichromacy];
                let mut color_vision_type_iterator =
                    ColorVisionTypeIterator::new(&color_vision_types[0]);
                while let Some(color_vision_type) = color_vision_type_iterator.next() {
                    color_vision_types.push(color_vision_type);
                }
                let choices = color_vision_types
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                Err(format!("Valid ColorVisionType options are:\n{}", choices))
            }
        }
    }
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
