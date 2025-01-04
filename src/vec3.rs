use std::ops;

use crate::rtweekend::{random_f64, random_f64_bounded};
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }
    pub fn y(self) -> f64 {
        self.e[1]
    }
    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn random_unit() -> Vec3 {
        let mut p;
        let mut lensq;

        loop {
            p = Vec3::random_bounded(-1.0, 1.0);
            lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                break;
            }
        }
        p / f64::sqrt(lensq)
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_f64(), random_f64(), random_f64()],
        }
    }
    pub fn random_bounded(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_f64_bounded(min, max),
                random_f64_bounded(min, max),
                random_f64_bounded(min, max),
            ],
        }
    }
    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere: Vec3 = Vec3::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            return on_unit_sphere;
        }
        -on_unit_sphere
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

// vec3 = vec1 / vec2
impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}
// vec1 /= vec2
impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

//////////////////////////////////////////////////////////////////
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}
impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] + other, self.e[1] + other, self.e[2] + other)
    }
}
impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] - other, self.e[1] - other, self.e[2] - other)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}
//
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}
impl ops::Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, vec: Vec3) -> Vec3 {
        vec + self
    }
}
impl ops::Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, vec: Vec3) -> Vec3 {
        vec - self
    }
}
impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, vec: Vec3) -> Vec3 {
        vec / self
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        -1.0 * self
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1.e[0], 1.0);
        assert_eq!(v1.e[1], 2.0);
        assert_eq!(v1.e[2], 3.0);

        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), 2.0);
        assert_eq!(v1.z(), 3.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let result = v1 + v2;

        assert_eq!(result.e[0], 2.0);
        assert_eq!(result.e[1], 4.0);
        assert_eq!(result.e[2], 6.0);
    }

    #[test]
    fn test_index() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1[0], 1.0);
        assert_eq!(v1[1], 2.0);
        assert_eq!(v1[2], 3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        v1 += v2;

        assert_eq!(v1.e[0], 2.0);
        assert_eq!(v1.e[1], 4.0);
        assert_eq!(v1.e[2], 6.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let result = v1 - v2;

        assert_eq!(result.e[0], 0.0);
        assert_eq!(result.e[1], 0.0);
        assert_eq!(result.e[2], 0.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        v1 -= v2;

        assert_eq!(v1.e[0], 0.0);
        assert_eq!(v1.e[1], 0.0);
        assert_eq!(v1.e[2], 0.0);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let result = v1 * v2;

        assert_eq!(result.e[0], 1.0);
        assert_eq!(result.e[1], 4.0);
        assert_eq!(result.e[2], 9.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        v1 *= v2;

        assert_eq!(v1.e[0], 1.0);
        assert_eq!(v1.e[1], 4.0);
        assert_eq!(v1.e[2], 9.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let result = v1 / v2;

        assert_eq!(result.e[0], 1.0);
        assert_eq!(result.e[1], 1.0);
        assert_eq!(result.e[2], 1.0);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        v1 /= v2;

        assert_eq!(v1.e[0], 1.0);
        assert_eq!(v1.e[1], 1.0);
        assert_eq!(v1.e[2], 1.0);
    }

    #[test]
    fn test_len() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
        assert_eq!(v1.length(), f64::sqrt(14.0));
    }

    #[test]
    fn test_operators() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let result = v1 + 1.0;

        assert_eq!(result.e[0], 2.0);
        assert_eq!(result.e[1], 3.0);
        assert_eq!(result.e[2], 4.0);

        let result = 1.0 + v1;

        assert_eq!(result.e[0], 2.0);
        assert_eq!(result.e[1], 3.0);
        assert_eq!(result.e[2], 4.0);

        let result = -v1;
        assert_eq!(result.e[0], -1.0);
        assert_eq!(result.e[1], -2.0);
        assert_eq!(result.e[2], -3.0);
    }
}
