use std::ops;

#[derive(PartialEq, Debug)]
pub struct Tuple(pub f32, pub f32, pub f32, pub f32);

impl Tuple {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 0.0)
    }

    pub fn color(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b, 1.0)
    }

    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }

    pub fn mag(&self) -> f32 {
        (self.0.powf(2.0) + self.1.powf(2.0) + self.2.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.mag();
        Self::vector(self.0 / mag, self.1 / mag, self.2 / mag)
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn cross(&self, other: &Tuple) -> Self {
        Self::vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn hadamard(&self, other: &Tuple) -> Self {
        Self(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Self;

    fn add(self, _rhs: Tuple) -> Self {
        Self(
            self.0 + _rhs.0,
            self.1 + _rhs.1,
            self.2 + _rhs.2,
            self.3 + _rhs.3,
        )
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Self;

    fn sub(self, _rhs: Tuple) -> Self {
        Self(
            self.0 - _rhs.0,
            self.1 - _rhs.1,
            self.2 - _rhs.2,
            self.3 - _rhs.3,
        )
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs, self.3 * _rhs)
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self {
        Self(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs, self.3 / _rhs)
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}
