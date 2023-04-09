use std::ops;

use crate::utility::{random, random_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn dot(self, o: Vec3) -> f64 {
        self[0] * o[0] + self[1] * o[1] + self[2] * o[2]
    }

    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self[1] * o[2] - self[2] * o[1],
            self[2] * o[0] - self[0] * o[2],
            self[0] * o[1] - self[1] * o[0],
        )
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn random() -> Self {
        Vec3::new(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_range(-1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;

        (self[0].abs() < s) && (self[1].abs() < s) && (self[2].abs() < s) 
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        return v - 2.0 * Self::dot(v, n) * n;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.e[i]
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, o: Vec3) -> Self {
        Vec3::new(self[0] + o[0], self[1] + o[1], self[2] + o[2])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, o: Vec3) {
        self[0] += o[0];
        self[1] += o[1];
        self[2] += o[2];
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, o: f64) -> Self {
        Vec3::new(self[0] + o, self[1] + o, self[2] + o)
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, o: f64) {
        self[0] += o;
        self[1] += o;
        self[2] += o;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, o: Vec3) -> Self {
        Vec3::new(self[0] - o[0], self[1] - o[1], self[2] - o[2])
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Vec3) {
        self[0] -= o[0];
        self[1] -= o[1];
        self[2] -= o[2];
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Vec3::new(self[0] * t, self[1] * t, self[2] * t)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, t: Vec3) -> Vec3 {
        t * self
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Vec3 {
        Vec3::new(self[0] * o[0], self[1] * o[1], self[2] * o[2])
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self[0] *= t;
        self[1] *= t;
        self[2] *= t;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}
