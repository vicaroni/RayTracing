use std::{ops, fmt::Display};

use crate::rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub const fn new() -> Self {
        Self { e: [0., 0., 0.] }
    }

    pub const fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self::from(u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
    }

    pub fn unit_vector(v: Self) -> Self {
        let len = v.length();
        v / len
    }

    pub fn random() -> Self {
        Self { e: [rng::gen(), rng::gen(), rng::gen()] }
    }

    pub fn random_in(min: f64, max: f64) -> Self {
        Self { e: [rng::sample(min, max), rng::sample(min, max), rng::sample(min, max)]}
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in(-1., 1.);
            if p.length_squared() < 1. { return p; }
        }
    }

    #[cfg(feature = "random_in_hemisphere")]
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(in_unit_sphere, *normal) > 0. { in_unit_sphere } else { -in_unit_sphere }
    }

    #[cfg(feature = "random_unit_vector")]
    pub fn random_unit_vector() -> Self {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x().abs() < S as f64 && self.y().abs() < S as f64 && self.z().abs() < S as f64
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2. * Self::dot(*v, *n) * n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_etat: f64) -> Self {
        let cos = Self::dot(-*uv, *n).min(1.);
        let r_out_perp = etai_etat * (*uv + cos * n);
        let r_out_par = -(1. - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_par
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output::from(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::from(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::from(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::from(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::from(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        *rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1./rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1./rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

pub type Color = Vec3;

pub fn write_color(color: Color, samples: usize) {
    let clamp = |n: f64| {n.max(0.).min(0.999)};
    let scale = 1. / samples as f64;
    let (r, g, b) = ((color.x() * scale).sqrt(), (color.y() * scale).sqrt(), (color.z() * scale).sqrt());
    println!("{} {} {}", 256. * clamp(r), 256. * clamp(g), 256. * clamp(b))
}

pub type Point = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_dot() {
        let v = Vec3::from(1.2, 0.5, 1.);
        let w = Vec3::from(1.5, 0.2, 2.);
        assert_eq!(Vec3::dot(v, w), 3.9)
    }

    #[test]
    fn test_refract_normal() {
        let vec = Vec3::from(1., 0., 0.);
        let etai_etat = 1.;
        assert_eq!(Vec3::refract(&vec, &-vec, etai_etat), vec);
    }
}
