use crate::{
    vec3::{Point, Vec3},
    ray::Ray,
    hittable::{Hittable, HitRecord}
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl Sphere {
    pub const fn new() -> Self {
        Self { center: Point::new(), radius: 0. }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, *r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        if disc < 0. {
            false
        } else {
            let sqrtd = disc.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return false;
                }
            }
            rec.t = root;
            rec.p = r.at(root);
            rec.set_face_normal(r, &((rec.p - self.center) / self.radius));
            true
        }
    }
}