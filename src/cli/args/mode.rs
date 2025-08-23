use std::str::FromStr;

pub enum Mode {
    Simulate,
    Daltonize,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simulate" | "s" => Ok(Mode::Simulate),
            "daltonize" | "d" => Ok(Mode::Daltonize),
            _ => {
                const OPTIONS: [&str; 2] = ["simulate (s)", "daltonize (d)"];
                Err(format!(
                    "Valid ColorVisionType options are:\n{}",
                    OPTIONS.join(", ")
                ))
            }
        }
    }
}
