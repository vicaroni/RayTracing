use crate::vec3::{Vec3, Point};

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point,
    dir: Vec3
}

impl Ray {
    pub const fn new() -> Self {
        Self { orig: Point::new(), dir: Vec3::new() }
    }

    pub const fn ray(origin: Point, direction: Vec3) -> Self {
        Self { orig: origin, dir: direction}
    }

    pub fn origin(&self) -> &Point { &self.orig }
    pub fn direction(&self) -> &Vec3 { &self.dir }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}