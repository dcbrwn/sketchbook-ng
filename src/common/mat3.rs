use std::ops;
use super::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3 {
    pub a11: f64, pub a12: f64, pub a13: f64,
    pub a21: f64, pub a22: f64, pub a23: f64,
    pub a31: f64, pub a32: f64, pub a33: f64,
}

impl<'a> ops::Mul<&'a Vec3> for &'a Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.a11 * rhs.x + self.a12 * rhs.y + self.a13 * rhs.z,
            y: self.a21 * rhs.x + self.a22 * rhs.y + self.a23 * rhs.z,
            z: self.a31 * rhs.x + self.a32 * rhs.y + self.a33 * rhs.z,
        }
    }
}

impl<'a> ops::Mul<&'a Mat3> for &'a Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: &'a Mat3) -> Mat3 {
        Mat3 {
            a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 + rhs.a31,
            a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 + rhs.a32,
            a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 + rhs.a33,

            a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 + rhs.a31,
            a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 + rhs.a32,
            a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 + rhs.a33,

            a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 + rhs.a31,
            a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 + rhs.a32,
            a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 + rhs.a33,
        }
    }
}

impl Mat3 {
    pub fn identity() -> Self {
        Mat3 {
            a11: 1.0, a12: 0.0, a13: 0.0,
            a21: 0.0, a22: 1.0, a23: 0.0,
            a31: 0.0, a32: 0.0, a33: 1.0,
        }
    }

    pub fn rotate(self, rad: f64) -> Self {
        &self * &Mat3 {
            a11: f64::cos(rad), a12: f64::sin(rad), a13: 0.0,
            a21: -f64::sin(rad), a22: f64::cos(rad), a23: 0.0,
            a31: 0.0, a32: 0.0, a33: 1.0,
        }
    }

    pub fn translate(self, to: Vec3) -> Self {
        &self * &Mat3 {
            a11: 1.0, a12: 0.0, a13: to.x,
            a21: 0.0, a22: 1.0, a23: to.y,
            a31: 0.0, a32: 0.0, a33: 1.0,
        }
    }

    pub fn scale(self, scale: Vec3) -> Self {
        &self * &Mat3 {
            a11: scale.x, a12: 0.0, a13: 0.0,
            a21: 0.0, a22: scale.y, a23: 0.0,
            a31: 0.0, a32: 0.0, a33: 1.0,
        }
    }
}
