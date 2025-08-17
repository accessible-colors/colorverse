use super::ColorVision;

// todo: suppress compiler warn
pub struct ColorVisionIterator {
    pub current: ColorVision,
    pub start: ColorVision,
}

impl ColorVisionIterator {
    /// new
    pub fn new(color_vision: &ColorVision) -> Self {
        Self {
            current: color_vision.to_owned(),
            start: color_vision.to_owned(),
        }
    }

    /// next
    pub fn next(&mut self) -> Option<ColorVision> {
        let next = match self.current {
            ColorVision::Trichromacy => ColorVision::Protanomaly,
            ColorVision::Protanomaly => ColorVision::Deuteranomaly,
            ColorVision::Protanopia => ColorVision::Deuteranopia,
            ColorVision::Deuteranomaly => ColorVision::Tritanomaly,
            ColorVision::Deuteranopia => ColorVision::Tritanopia,
            ColorVision::Tritanomaly => ColorVision::Achromatomaly,
            ColorVision::Tritanopia => ColorVision::Achromatopsia,
            ColorVision::Achromatomaly => ColorVision::Protanopia,
            ColorVision::Achromatopsia => ColorVision::Trichromacy,
        };

        if next == self.start {
            None
        } else {
            self.current = next.clone();
            Some(next)
        }
    }
}
