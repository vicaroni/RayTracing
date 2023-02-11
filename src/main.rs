use std::{
    io::{stderr, Write},
    rc::Rc
};

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
mod rng;

use vec3::{Color, write_color, Vec3, Point};
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;
use rng::gen;

fn ray_color(r: Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        Color::new()
    } else if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
        let (mut scattered, mut attenuation) = (Ray::new(), Vec3::new());
        if rec.mat_ptr.as_ref().unwrap().scatter(&r, &rec, &mut attenuation, &mut scattered) {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Color::new()
        }
    } else {
        let unit_direction = Vec3::unit_vector(*r.direction());
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Color::from(1., 1., 1.) +
            t * Color::from(0.5, 0.7, 1.)
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u8 = 100;
    const MAX_DEPTH: u8 = 100;
    let r = (std::f64::consts::PI / 4.).cos();

    let (mat_left, mat_right) = (
        Rc::new(material::Lambertian { albedo: Color::from(0., 0., 1.)}),
        Rc::new(material::Lambertian { albedo: Color::from(1., 0., 0.)}),
    );

    let world = HittableList::with(vec![
        Rc::new(Sphere { center: Point::from(-r, 0., -1.), radius: r, mat_ptr: mat_left}),
        Rc::new(Sphere { center: Point::from(r, 0., -1.), radius: r, mat_ptr: mat_right})
    ]);

    let cam = Camera::new(90., ASPECT_RATIO);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let (u, v) = ((i as f64 + gen::<f64>()) / (IMAGE_WIDTH - 1) as f64,
                    (j as f64 + gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &world, MAX_DEPTH as usize);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as usize);
        }
    }
    eprintln!("\nDone");
}
