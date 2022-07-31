use std::ops;

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

    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (self.x() * 255.999) as u16,
            (self.y() * 255.999) as u16,
            (self.z() * 255.999) as u16
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
