use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    vec3::{Vec3, Point},
    ray::Ray,
    material::Material
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub mat_ptr: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(*r.direction(), *outward_normal) < 0.;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }

    pub fn from(p: Point, t: f64, r: &Ray, outward_normal: &Vec3, mat_ptr: Arc<dyn Material>) -> Self {
        let mut ret = Self { p, normal: Vec3::new(), mat_ptr: Some(mat_ptr), t, front_face: false };
        ret.set_face_normal(r, outward_normal);
        ret
    }
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub const fn with(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}