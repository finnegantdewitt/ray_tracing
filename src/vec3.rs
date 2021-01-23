use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from(
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::from(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.e[idx]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.e[idx]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::from(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vec3::from(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        (1.0 / other) * self
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::from(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self::from(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

pub type point3 = Vec3;
pub type color = Vec3;
