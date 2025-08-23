use super::ColorVisionType;

// todo: suppress compiler warn
pub struct ColorVisionTypeIterator {
    pub current: ColorVisionType,
    pub start: ColorVisionType,
}

impl ColorVisionTypeIterator {
    /// new
    pub fn new(color_vision: &ColorVisionType) -> Self {
        Self {
            current: color_vision.to_owned(),
            start: color_vision.to_owned(),
        }
    }

    /// next
    pub fn next(&mut self) -> Option<ColorVisionType> {
        let next = match self.current {
            ColorVisionType::Trichromacy => ColorVisionType::Protanomaly,
            ColorVisionType::Protanomaly => ColorVisionType::Deuteranomaly,
            ColorVisionType::Deuteranomaly => ColorVisionType::Tritanomaly,
            ColorVisionType::Tritanomaly => ColorVisionType::Achromatomaly,
            ColorVisionType::Achromatomaly => ColorVisionType::Trichromacy,
        };

        if next == self.start {
            None
        } else {
            self.current = next.clone();
            Some(next)
        }
    }
}
