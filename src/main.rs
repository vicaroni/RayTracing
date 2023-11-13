use std::{
    rc::Rc,
    time::Instant
};

use winit::{
    dpi::LogicalSize,
    event::{
        Event,
        WindowEvent
    },
    event_loop::EventLoop,
    window::WindowBuilder
};
use threadpool::ThreadPool;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
mod rng;
mod window;

use vec3::{
    Color,
    Vec3,
    Point
};
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;
use window::GraphicsContext;

fn random_scene() -> HittableList {
    let mut world = HittableList::with(vec![
        Rc::new(Sphere {center: Point::from(0., -1000., 0.), radius: 1000.,
            mat_ptr: Rc::new(material::Lambertian { albedo: Color::from(0.5, 0.5, 0.5)})})
    ]);

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::from(a as f64 + 0.9 * rng::gen::<f64>(), 0.2, b as f64 + 0.9 * rng::gen::<f64>());
            if (center - Point::from(4., 0.2, 0.)).length() > 0.9 {
                world.add(Rc::new(Sphere { center, radius: 0.2, mat_ptr: match rng::gen::<f64>() {
                    x if x < 0.8 => Rc::new(material::Lambertian { albedo: Color::random() * Color::random()}),
                    x if x < 0.95 => Rc::new(material::Metal { albedo: Color::random_in(0.5, 1.), fuzz: rng::sample(0., 0.5)}),
                    _ => Rc::new(material::Dielectric {ir: 1.5})
                }}));
            }
        }
    }

    world.add(Rc::new(Sphere { center: Point::from(0., 1., 0.), radius: 1., mat_ptr: Rc::new(material::Dielectric { ir: 1.5 })}));
    world.add(Rc::new(Sphere { center: Point::from(-4., 1., 0.), radius: 1., mat_ptr: Rc::new(material::Lambertian { albedo: Color::from(0.4, 0.2, 0.1)})}));
    world.add(Rc::new(Sphere { center: Point::from(4., 1., 0.), radius: 1., mat_ptr: Rc::new(material::Metal::new(0.7, 0.6, 0.5, 0.))}));
    world
}

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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u16 = 1200;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u16 = 500;
    const MAX_DEPTH: u8 = 50;

    let world = random_scene();

    let (lookfrom, lookat) = (
        Point::from(13., 2., 3.),
        Point::from(0., 0., 0.)
    );
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::from(0., 1., 0.),
        20.,
        ASPECT_RATIO,
        0.1,
        10.
    );

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Ray tracing in one weekend")
        .with_inner_size(LogicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT))
        .build(&event_loop).unwrap();

    let mut ctx = GraphicsContext::new(&window);

    // let pool = ThreadPool::new(8);
    let mut i = 0;
    let start = Instant::now();

    // for i in 0..IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize {
    //     pool.execute(||{

    //     })
    // }

    event_loop.run(move |event, elwt|{
        match event {
            Event::WindowEvent { event: WindowEvent::RedrawRequested, ..} => {
                let mut pixel_color = Color::new();
                // println!("Drawing {} / {}", i, IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let (u, v) = (((i % IMAGE_WIDTH as usize) as f64 + rng::gen::<f64>()) / (IMAGE_WIDTH - 1) as f64,
                        ((IMAGE_HEIGHT as usize - (i / IMAGE_WIDTH as usize)) as f64 + rng::gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(r, &world, MAX_DEPTH as usize);
                }
                window.pre_present_notify();
                ctx.draw_pixel(i, pixel_color, SAMPLES_PER_PIXEL);
                i += 1;
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => elwt.exit(),
            Event::AboutToWait =>
                if i < IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize {
                    window.request_redraw();
                } else {
                    println!("Done, took {:?}", start.elapsed());
                },
            _ => {}
        }
    }).unwrap();
}
