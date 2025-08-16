use crate::core::vec3::Vec3;

/// color basis to mix the RGB channels to obtain output colors
#[derive(Clone, Default, Copy, Debug)]
pub struct ColorBasis {
    /// percentages of red, green and blue to mix on the red channel
    pub red: Vec3,
    /// percentages of red, green and blue to mix on the green channel
    pub green: Vec3,
    /// percentages of red, green and blue to mix on the blue channel
    pub blue: Vec3,
}

impl ColorBasis {
    /// create with default
    pub fn new() -> Self {
        Self {
            red: Vec3::default(),
            green: Vec3::default(),
            blue: Vec3::default(),
        }
    }

    /// create with parameters
    pub fn from(red: Vec3, green: Vec3, blue: Vec3) -> Self {
        Self { red, green, blue }
    }
}
