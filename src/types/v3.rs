use super::ray::{random_float, random_float_range};
use std::ops;

use super::ray::clamp;

pub type Point3 = V3;
pub type Color = V3;

#[derive(Debug, Clone)]
pub struct V3 {
    pub values: [f64; 3],
}

impl Default for V3 {
    fn default() -> Self {
        Self {
            values: [0.0, 0.0, 0.0],
        }
    }
}

impl V3 {
    pub fn new(val: [f64; 3]) -> Self {
        Self { values: val }
    }

    pub fn random() -> Self {
        Self {
            values: [random_float(), random_float(), random_float()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            values: [
                random_float_range(min, max),
                random_float_range(min, max),
                random_float_range(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> V3 {
        loop {
            let p = V3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> V3 {
        V3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &V3) -> V3 {
        let us = V3::random_in_unit_sphere();
        if us.dot(normal) > 0.0 {
            return us;
        }
        -us
    }

    pub fn x(&self) -> f64 {
        self.values[0]
    }

    pub fn y(&self) -> f64 {
        self.values[1]
    }

    pub fn z(&self) -> f64 {
        self.values[2]
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, other: &V3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross(&self, other: V3) -> Self {
        Self {
            values: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    pub fn write_color(&self, samples_per_pixel: u64) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples_per_pixel as f64;

        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);

        println!(
            "{} {} {}",
            (clamp(r, 0.0, 0.999) * 256.0) as u16,
            (clamp(g, 0.0, 0.999) * 256.0) as u16,
            (clamp(b, 0.0, 0.999) * 256.0) as u16
        )
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
}

impl ops::Neg for V3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            values: [-self.values[0], -self.values[1], -self.values[2]],
        }
    }
}

impl ops::Add<V3> for V3 {
    type Output = Self;
    fn add(self, _rhs: V3) -> Self {
        Self {
            values: [
                self.values[0] + _rhs.values[0],
                self.values[1] + _rhs.values[1],
                self.values[2] + _rhs.values[2],
            ],
        }
    }
}

impl ops::Add<&V3> for V3 {
    type Output = Self;
    fn add(self, _rhs: &V3) -> Self {
        Self {
            values: [
                self.values[0] + _rhs.values[0],
                self.values[1] + _rhs.values[1],
                self.values[2] + _rhs.values[2],
            ],
        }
    }
}

impl ops::Add<&V3> for &V3 {
    type Output = V3;
    fn add(self, _rhs: &V3) -> V3 {
        V3 {
            values: [
                self.values[0] + _rhs.values[0],
                self.values[1] + _rhs.values[1],
                self.values[2] + _rhs.values[2],
            ],
        }
    }
}

impl ops::Sub<V3> for V3 {
    type Output = Self;
    fn sub(self, _rhs: V3) -> Self {
        Self {
            values: [
                self.values[0] - _rhs.values[0],
                self.values[1] - _rhs.values[1],
                self.values[2] - _rhs.values[2],
            ],
        }
    }
}

impl ops::Sub<&V3> for &V3 {
    type Output = V3;
    fn sub(self, _rhs: &V3) -> V3 {
        V3 {
            values: [
                self.values[0] - _rhs.values[0],
                self.values[1] - _rhs.values[1],
                self.values[2] - _rhs.values[2],
            ],
        }
    }
}

impl ops::Sub<V3> for &V3 {
    type Output = V3;
    fn sub(self, _rhs: V3) -> V3 {
        V3 {
            values: [
                self.values[0] - _rhs.values[0],
                self.values[1] - _rhs.values[1],
                self.values[2] - _rhs.values[2],
            ],
        }
    }
}

impl ops::Sub<&V3> for V3 {
    type Output = V3;
    fn sub(self, _rhs: &V3) -> V3 {
        V3 {
            values: [
                self.values[0] - _rhs.values[0],
                self.values[1] - _rhs.values[1],
                self.values[2] - _rhs.values[2],
            ],
        }
    }
}

impl ops::Mul<f64> for V3 {
    type Output = Self;
    fn mul(self, _rhs: f64) -> Self {
        Self {
            values: [
                self.values[0] * _rhs,
                self.values[1] * _rhs,
                self.values[2] * _rhs,
            ],
        }
    }
}

impl ops::Mul<&V3> for f64 {
    type Output = V3;
    fn mul(self, _rhs: &V3) -> V3 {
        V3 {
            values: [self * _rhs.x(), self * _rhs.y(), self * _rhs.z()],
        }
    }
}

impl ops::Div<f64> for V3 {
    type Output = Self;
    fn div(self, _rhs: f64) -> Self {
        Self {
            values: [
                self.values[0] / _rhs,
                self.values[1] / _rhs,
                self.values[2] / _rhs,
            ],
        }
    }
}

impl ops::Div<f64> for &V3 {
    type Output = V3;
    fn div(self, _rhs: f64) -> V3 {
        V3 {
            values: [
                self.values[0] / _rhs,
                self.values[1] / _rhs,
                self.values[2] / _rhs,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ctor() {
        let dc = V3::default();
        assert_eq!(dc.values[0], 0.0);
        assert_eq!(dc.values[1], 0.0);
        assert_eq!(dc.values[2], 0.0);
    }

    #[test]
    fn getters() {
        let dc = V3::new([1.0, 2.0, 3.0]);
        assert_eq!(dc.x(), 1.0);
        assert_eq!(dc.y(), 2.0);
        assert_eq!(dc.z(), 3.0);
    }

    #[test]
    fn add() {
        let dc = V3::new([1.0, 2.0, 3.0]);
        let dc1 = V3::new([1.0, 2.0, 3.0]);
        let dc2 = dc1 + dc;
        assert_eq!(dc2.x(), 2.0);
        assert_eq!(dc2.y(), 4.0);
        assert_eq!(dc2.z(), 6.0);
    }

    #[test]
    fn multiply() {
        let dc = V3::new([1.0, 2.0, 3.0]);
        let dc1 = dc * 2.0;
        assert_eq!(dc1.x(), 2.0);
        assert_eq!(dc1.y(), 4.0);
        assert_eq!(dc1.z(), 6.0);
    }

    #[test]
    fn negative() {
        let dc = V3::new([1.0, 2.0, 3.0]);
        assert_eq!(-dc.x(), -1.0);
        assert_eq!(-dc.y(), -2.0);
        assert_eq!(-dc.z(), -3.0);
    }

    #[test]
    fn divide() {
        let dc = V3::new([3.0, 6.0, 9.0]);
        let dc1 = dc / 3.0;
        assert_eq!(dc1.x(), 1.0);
        assert_eq!(dc1.y(), 2.0);
        assert_eq!(dc1.z(), 3.0);
    }
}
