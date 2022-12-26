use core::fmt::Debug;
use std::rc::Rc;

use crate::{
    vec3::{Vec3, Point},
    ray::Ray
};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub const fn new() -> Self {
        Self { p: Point::new(), normal: Vec3::new(), t: 0., front_face: false }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(*r.direction(), *outward_normal) < 0.;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }

    pub fn from(p: Point, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let mut ret = Self { p, normal: Vec3::new(), t, front_face: false };
        ret.set_face_normal(r, outward_normal);
        ret
    }
}

pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub const fn new() -> Self {
        Self { objects: vec![] }
    }

    pub const fn with(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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