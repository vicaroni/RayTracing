use crate::{
    vec3::{Point, Vec3},
    ray::Ray
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, v_fov: f64, aspect_ratio: f64) -> Self {
        let viewport_height = 2. * (v_fov.to_radians() / 2.).tan();
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let (horizontal, vertical) = (
            aspect_ratio * viewport_height * u,
            viewport_height * Vec3::cross(w, u)
        );
        Self {
            origin: lookfrom,
            lower_left_corner: lookfrom - horizontal / 2. - vertical / 2. - w,
            horizontal, vertical
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::ray(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}