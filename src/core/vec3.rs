/// three dimension Vector
#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vec3 {
    /// from
    fn from(arr: [f32; 3]) -> Self {
        Vec3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl Vec3 {
    /// multiplication of a vector and a scalar
    pub fn scale(&self, s: f32) -> Self {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    /// add two vectors
    pub fn add(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
