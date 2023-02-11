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
    pub fn new(v_fov: f64, aspect_ratio: f64) -> Self {
        let viewport_height = 2. * (v_fov.to_radians() / 2.).tan();
        let (origin, horizontal, vertical) = (
            Point::new(), Vec3::from(aspect_ratio * viewport_height, 0., 0.), Vec3::from(0., viewport_height, 0.)
        );
        Self {
            origin,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - Vec3::from(0., 0., 1.),
            horizontal, vertical
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::ray(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}