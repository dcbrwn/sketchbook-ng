use std::ops;
use super::vec3::Vec3;

#[derive(Clone)]
pub struct Mat3 {
    pub m11: f64, pub m12: f64, pub m13: f64,
    pub m21: f64, pub m22: f64, pub m23: f64,
    pub m31: f64, pub m32: f64, pub m33: f64,
}

impl<'a> ops::Mul<&'a Vec3> for &'a Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            y: self.m21 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            z: self.m31 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        }
    }
}

impl<'a> ops::Mul<&'a Mat3> for &'a Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: &'a Mat3) -> Mat3 {
        Mat3 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,

            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,

            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}

impl Mat3 {
    pub fn identity() -> Self {
        Mat3 {
            m11: 1.0, m12: 0.0, m13: 0.0,
            m21: 0.0, m22: 1.0, m23: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn close_to(&self, other: &Mat3, e: f64) -> bool {
        f64::abs(self.m11 - other.m11) < e &&
        f64::abs(self.m12 - other.m12) < e &&
        f64::abs(self.m13 - other.m13) < e &&
        f64::abs(self.m21 - other.m21) < e &&
        f64::abs(self.m22 - other.m22) < e &&
        f64::abs(self.m23 - other.m23) < e &&
        f64::abs(self.m31 - other.m31) < e &&
        f64::abs(self.m32 - other.m32) < e &&
        f64::abs(self.m33 - other.m33) < e
    }

    pub fn rotate(&self, rad: f64) -> Self {
        self * &Mat3 {
            m11: f64::cos(rad), m12: f64::sin(rad), m13: 0.0,
            m21: -f64::sin(rad), m22: f64::cos(rad), m23: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn translate(&self, to: &Vec3) -> Self {
        self * &Mat3 {
            m11: 1.0, m12: 0.0, m13: to.x,
            m21: 0.0, m22: 1.0, m23: to.y,
            m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn scale(&self, scale: &Vec3) -> Self {
        self * &Mat3 {
            m11: scale.x, m12: 0.0, m13: 0.0,
            m21: 0.0, m22: scale.y, m23: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn inverse(&mut self) -> bool {
        // Code from: https://docs.rs/nalgebra/0.21.1/src/nalgebra/linalg/inverse.rs.html#31-119

        let Mat3 {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        } = *self;

        let minor_m12_m23 = m22 * m33 - m32 * m23;
        let minor_m11_m23 = m21 * m33 - m31 * m23;
        let minor_m11_m22 = m21 * m32 - m31 * m22;

        let determinant =
            m11 * minor_m12_m23 - m12 * minor_m11_m23 + m13 * minor_m11_m22;

        if determinant == 0.0 {
            false
        } else {
            self.m11 = minor_m12_m23 / determinant;
            self.m12 = (m13 * m32 - m33 * m12) / determinant;
            self.m13 = (m12 * m23 - m22 * m13) / determinant;
            self.m21 = -minor_m11_m23 / determinant;
            self.m22 = (m11 * m33 - m31 * m13) / determinant;
            self.m23 = (m13 * m21 - m23 * m11) / determinant;
            self.m31 = minor_m11_m22 / determinant;
            self.m32 = (m12 * m31 - m32 * m11) / determinant;
            self.m33 = (m11 * m22 - m21 * m12) / determinant;

            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mat3;
    use super::Vec3;

    #[test]
    fn mat_mul_vec() {
        let vec = Vec3 { x: 10.0, y: 20.0, z: 30.0 };
        let mat = Mat3 {
            m11: 1.0, m12: 3.1, m13: 0.0,
            m21: 0.0, m22: 5.2, m23: 0.3,
            m31: 8.0, m32: 7.4, m33: 1.0,
        };

        assert_eq!(&mat * &vec, Vec3 { x: 72.0, y: 113.0, z: 258.0 });
    }

    #[test]
    fn mat_mul_mat() {
        let mat1 = Mat3 {
            m11: 6.0, m12: 3.6, m13: 1.0,
            m21: 9.4, m22: 1.2, m23: 0.0,
            m31: 0.3, m32: 6.7, m33: 5.2,
        };

        let mat2 = Mat3 {
            m11: 1.0, m12: 3.1, m13: 0.0,
            m21: 0.0, m22: 5.2, m23: 0.3,
            m31: 8.0, m32: 7.4, m33: 1.0,
        };

        let actual = &mat1 * &mat2;

        let expected = Mat3 {
            m11: 14.0, m12: 44.72, m13: 2.08,
            m21: 9.4, m22: 35.38, m23: 0.36,
            m31: 41.9, m32: 74.25, m33: 7.21,
        };

        assert!(actual.close_to(&expected, 0.00001));
    }

    #[test]
    fn test_transform() {
        let transform = Mat3::identity()
            .translate(&Vec3 { x: 15.0, y: 15.0, z: 1.0 });

        let point = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        let transformed = &transform * &point;

        assert_eq!(transformed, Vec3 { x: 15.0, y: 15.0, z: 1.0 });
    }

    #[test]
    fn test_inverse() {
        let transform = Mat3::identity()
            .translate(&Vec3 { x: 15.0, y: 15.0, z: 1.0 });

        let mut inverse_transform = transform.clone();
        inverse_transform.inverse();

        let point = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        let transformed = &(&transform * &inverse_transform) * &point;

        assert_eq!(transformed, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    }
}
