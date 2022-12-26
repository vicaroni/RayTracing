use std::{
    io::{stderr, Write},
    rc::Rc
};

use rand::{
    Rng, SeedableRng,
    rngs::SmallRng
};

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

use vec3::{Color, write_color, Vec3, Point};
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;

fn ray_color(r: Ray, world: &impl Hittable, rng: &mut impl Rng, depth: usize) -> Color {
    if depth == 0 {
        Color::new()
    } else if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
        #[cfg(feature = "random_in_hemisphere")]
        let target = rec.p + rec.normal + Vec3::random_in_hemisphere(&rec.normal, rng);

        #[cfg(feature = "random_unit_vector")]
        let target = rec.p + rec.normal + Vec3::random_unit_vector(rng);
        
        0.5 * ray_color(Ray::ray(rec.p, target - rec.p), world, rng, depth - 1)
    } else {
        let unit_direction = Vec3::unit_vector(*r.direction());
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Color::from(1., 1., 1.) +
            t * Color::from(0.5, 0.7, 1.)
    }
}

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u8 = 100;
    const MAX_DEPTH: u8 = 50;

    let world = HittableList::with(vec![
        Rc::new(Sphere { center: Point::from(0., 0., -1.), radius: 0.5 }),
        Rc::new(Sphere { center: Point::from(0., -100.5, -1.), radius: 100. })
    ]);

    let cam = Camera::new();
    let mut rng = SmallRng::from_entropy();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let (u, v) = ((i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64,
                    (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &world, &mut rng, MAX_DEPTH as usize);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as usize);
        }
    }
    eprintln!("\nDone");
}
