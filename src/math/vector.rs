use num::{NumCast, ToPrimitive};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new<T: ToPrimitive>(x: T, y: T) -> Self {
        Self {
            x: NumCast::from(x).unwrap(),
            y: NumCast::from(y).unwrap(),
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        let angle = angle * PI / 180.;
        let cs = angle.cos();
        let sn = angle.sin();

        Vector2 {
            x: self.x * cs - self.y * sn,
            y: self.x * sn + self.y * cs,
        }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
