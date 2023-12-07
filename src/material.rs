use core::fmt::Debug;

use crate::{
    ray::Ray,
    hittable::HitRecord,
    vec3::{Color, Vec3},
    rng
};

pub trait Material: Debug + Send + Sync{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut direction = rec.normal + Vec3::random_unit_vector();
        if direction.near_zero() {
            direction = rec.normal;
        }
        *scattered = Ray::ray(rec.p, direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo_r: f64, albedo_g: f64, albedo_b: f64, fuzz: f64) -> Self {
        Self { albedo: Color::from(albedo_r, albedo_g, albedo_b), fuzz: if fuzz < 1. { fuzz } else { 1. }}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(*r_in.direction()), &rec.normal);
        *scattered = Ray::ray(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(*scattered.direction(), rec.normal) > 0.
    }
}

#[derive(Debug)]
pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::from(1., 1., 1.);
        let refraction_ratio = if rec.front_face { 1. / self.ir } else { self.ir };

        let unit_direction = Vec3::unit_vector(*r_in.direction());
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1. ||
                Self::reflectance(cos_theta, refraction_ratio) > rng::gen::<f64>() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        *scattered = Ray::ray(rec.p, direction);
        true
    }
}
