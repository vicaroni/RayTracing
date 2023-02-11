use std::rc::Rc;

use crate::{
    vec3::{Point, Vec3},
    ray::Ray,
    hittable::{Hittable, HitRecord},
    material::Material
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>
}

// impl Sphere {
//     pub const fn new() -> Self {
//         Self { center: Point::new(), radius: 0. }
//     }
// }

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, *r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        if disc < 0. {
            None
        } else {
            let sqrtd = disc.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }
            let p = r.at(root);
            Some(HitRecord::from(p, root, r, &((p - self.center) / self.radius), self.mat_ptr.clone()))
        }
    }
}