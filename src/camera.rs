use crate::{
    vec3::{Point, Vec3},
    ray::Ray
};

pub struct Camera {
    lower_left_corner: Point,
}

impl Camera {
    const ORIGIN: Point = Point::new();
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const VIEWPORT_HEIGHT: f32 = 2.;
    const VIEWPORT_WIDTH: f32 = Camera::ASPECT_RATIO * Camera::VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.;
    const HORIZONTAL: Vec3 = Vec3::from(Camera::VIEWPORT_WIDTH as f64, 0., 0.);
    const VERTICAL: Vec3 = Vec3::from(0., Camera::VIEWPORT_HEIGHT as f64, 0.);

    pub fn new() -> Self {
        Self { lower_left_corner: Camera::ORIGIN - Camera::HORIZONTAL / 2. - Camera::VERTICAL / 2. - Vec3::from(0., 0., Camera::FOCAL_LENGTH as f64)}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::ray(Camera::ORIGIN, self.lower_left_corner + u * Camera::HORIZONTAL + v * Camera::VERTICAL - Camera::ORIGIN)
    }
}