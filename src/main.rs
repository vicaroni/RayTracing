use std::{
    sync::{
        Arc,
        mpsc::channel,
    },
    time::Instant
};

use once_cell::sync::Lazy;

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

const ASPECT_RATIO: f64 = 3.0 / 2.0;

static CAMERA: Lazy<Camera> = Lazy::new(|| Camera::new(
    Point::from(13., 2., 3.),
    Point::from(0., 0., 0.),
    Vec3::from(0., 1., 0.),
    20.,
    ASPECT_RATIO,
    0.1,
    10.
));

static WORLD: Lazy<HittableList> = Lazy::new(random_scene);

fn random_scene() -> HittableList {
    let mut world = HittableList::with(vec![
        Arc::new(Sphere {center: Point::from(0., -1000., 0.), radius: 1000.,
            mat_ptr: Arc::new(material::Lambertian { albedo: Color::from(0.5, 0.5, 0.5)})})
    ]);

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::from(a as f64 + 0.9 * rng::gen::<f64>(), 0.2, b as f64 + 0.9 * rng::gen::<f64>());
            if (center - Point::from(4., 0.2, 0.)).length() > 0.9 {
                world.add(Arc::new(Sphere { center, radius: 0.2, mat_ptr: match rng::gen::<f64>() {
                    x if x < 0.8 => Arc::new(material::Lambertian { albedo: Color::random() * Color::random()}),
                    x if x < 0.95 => Arc::new(material::Metal { albedo: Color::random_in(0.5, 1.), fuzz: rng::sample(0., 0.5)}),
                    _ => Arc::new(material::Dielectric {ir: 1.5})
                }}));
            }
        }
    }

    world.add(Arc::new(Sphere { center: Point::from(0., 1., 0.), radius: 1., mat_ptr: Arc::new(material::Dielectric { ir: 1.5 })}));
    world.add(Arc::new(Sphere { center: Point::from(-4., 1., 0.), radius: 1., mat_ptr: Arc::new(material::Lambertian { albedo: Color::from(0.4, 0.2, 0.1)})}));
    world.add(Arc::new(Sphere { center: Point::from(4., 1., 0.), radius: 1., mat_ptr: Arc::new(material::Metal::new(0.7, 0.6, 0.5, 0.))}));
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
    const IMAGE_WIDTH: u16 = 800;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u16 = 250;
    const SCALE: f64 = 255. / SAMPLES_PER_PIXEL as f64;
    const MAX_DEPTH: u8 = 50;

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Ray tracing in one weekend")
        .with_inner_size(LogicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT))
        .build(&event_loop).unwrap();

    let mut ctx = GraphicsContext::new(&window);

    let (tx, rx) = channel();
    let pool = ThreadPool::new(8);
    let start = Instant::now();

    for i in 0..IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize {
        let (cam, world, tx) = (&*CAMERA, &*WORLD, tx.clone());
        pool.execute(move ||{
            let mut color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let (u, v) = (((i % IMAGE_WIDTH as usize) as f64 + rng::gen::<f64>()) / (IMAGE_WIDTH - 1) as f64,
                    ((IMAGE_HEIGHT as usize - (i / IMAGE_WIDTH as usize)) as f64 + rng::gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64);
                    let r = cam.get_ray(u, v);
                    color += ray_color(r, world, MAX_DEPTH as usize);
            }
            tx.send((i, ((color.x() * SCALE) as u32) << 16 | ((color.y() * SCALE) as u32) << 8 | ((color.z() * SCALE) as u32))).expect("Canale in attesa");
        });
    }

    drop(tx);
    let (mut rx, mut rd) = (rx.iter().peekable(), 0usize);

    event_loop.run(move |event, elwt|{
        match event {
            Event::WindowEvent { event: WindowEvent::RedrawRequested, ..} => {
                window.pre_present_notify();
                for _ in 0..IMAGE_WIDTH {
                    let (i, pixel) = rx.next().expect("Errore ricezione");
                    ctx.draw_pixel(i, pixel);
                    rd += 1;
                    if rd == IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize {
                        println!("Took {:?}", start.elapsed());
                    }
                }
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
                std::process::exit(0);
            },
            Event::AboutToWait =>
                if rx.peek().is_some() {
                    window.request_redraw();
                },
            _ => {}
        }
    }).unwrap();
}
