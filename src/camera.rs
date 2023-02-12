use crate::{
    vec3::{Point, Vec3},
    ray::Ray
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3,
               v_fov: f64, aspect_ratio: f64,
               aperture: f64, focus_dist: f64) -> Self {
        let viewport_height = 2. * (v_fov.to_radians() / 2.).tan();
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);
        let (horizontal, vertical) = (
            focus_dist * aspect_ratio * viewport_height * u,
            focus_dist * viewport_height * v
        );
        Self {
            origin: lookfrom,
            lower_left_corner: lookfrom - horizontal / 2. - vertical / 2. - focus_dist * w,
            horizontal, vertical, u, v, w, lens_radius: aperture / 2.
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::ray(self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}