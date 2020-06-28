use std::f32;
use std::fmt;
use std::ops;

struct Vec2 {
    x: f32,
    y: f32,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
