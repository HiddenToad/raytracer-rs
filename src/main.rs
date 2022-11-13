mod camera;
mod collidable;
mod color;
mod material;
mod point;
mod ray;
mod utility;

pub use camera::*;
pub use collidable::*;
pub use color::*;
pub use material::*;
pub use point::*;
pub use ray::*;
pub use utility::*;

use once_cell::sync::Lazy;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    sync::mpsc,
    sync::{Arc, RwLock},
    thread,
};


static WORLD: Lazy<Arc<RwLock<CollidableVec>>> =
    Lazy::new(|| Arc::new(RwLock::new(CollidableVec::new())));

pub const ASPECT_RATIO: f64 = 1.;
pub const IMG_W: i32 = 450;
pub const IMG_H: i32 = (IMG_W as f64 / ASPECT_RATIO) as i32;

pub const SAMPLES: u32 = 150;
pub const MAX_DEPTH: i32 = 50;
pub const THREAD_INTERVAL: i32 = 150;
pub const NUM_THREADS: i32 = IMG_W / THREAD_INTERVAL;

fn main() {
    assert!(
        IMG_W % THREAD_INTERVAL == 0,
        "image width must be divisible by the thread interval"
    );
    assert!(
        IMG_H % THREAD_INTERVAL == 0,
        "image height must be divisible by the thread interval"
    );

    let material_ground = Lambertian::new_arc(Color::new(0.5, 0.5, 0.5));
    let material_solid = Lambertian::new_arc(Color::new(0.7, 0.3, 0.3));
    let material_metal = Metal::new_arc(Color::new(0.6, 0.6, 0.6), 0.15);
    let material_dielectric = Dielectric::new_arc(1.5);

    add_to_world(Sphere::boxed(
        Point::new(0.0, -1000., -1.0),
        1000.,
        material_ground.clone(),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let random = rand();
            let center = Point::new((a as f64) + 0.9 * rand(), 0.195, (b as f64) + 0.9 * rand());
            if (center - Point::new(4., 0.195, 0.)).len() > 0.9 {
                let material: Arc<dyn Material>;

                if random < 0.8 {
                    let albedo = Color::random() * Color::random();
                    material = Lambertian::new_arc(albedo);
                    add_to_world(Sphere::boxed(center, 0.2, material));
                } else {
                    let albedo = Color::rand_range(0.5, 1.);
                    let fuzz = rand_range(0., 0.35);
                    material = Metal::new_arc(albedo, fuzz);
                    add_to_world(Sphere::boxed(center, 0.2, material));
                }
            }
        }
    }

    let camera = Camera::new()
        .vfov(20.)
        .look_from(Point::new(8.2, 4.2, 3.))
        .look_at(Point::all(0.))
        .lens_radius(0.02);

    let mut data = vec![vec![Vec::<u8>::new(); THREAD_INTERVAL as usize]; NUM_THREADS as usize];
    let (sender, reciever) = mpsc::channel();

    for thread_num in 1..=NUM_THREADS {
        let sender = sender.clone();
        let camera = camera.clone();
        thread::spawn(move || {
            for i in (THREAD_INTERVAL * (thread_num - 1))..(THREAD_INTERVAL * thread_num) {
                for j in 0..IMG_W {
                    let mut color = Color::black();
                    for _ in 0..SAMPLES {
                        let x = (j as f64 + rand()) / (IMG_W - 1) as f64;
                        let y = (i as f64 + rand()) / (IMG_H - 1) as f64;
                        let ray = camera.get_ray(x, y);
                        color = color + ray.color();
                    }
                    sender.send((color.as_output(SAMPLES), thread_num)).unwrap();
                }
            }
        });
    }
    drop(sender);

    let mut counter = 0;
    let mut last_percent = -1.;
    while let Ok((bytes, thread_num)) = reciever.recv() {
        let percent = ((counter as f64 / (IMG_H * IMG_W) as f64) * 100.).floor();
        if percent != last_percent {
            println!("Progress: {percent}%");
            last_percent = percent;
        }

        data[(thread_num - 1) as usize].push(bytes);
        counter += 1;
    }

    println!("Progress: 100%, writing to file.");
    let mut buf: String;
    {
        let mut n = fs::File::open("rayout/.n.txt").unwrap_or_else(|_|{
            fs::create_dir("rayout").ok();
            fs::File::create("rayout/.n.txt").unwrap().write_all(b"0").unwrap();
            fs::File::open("rayout/.n.txt").unwrap()

        });

        buf = String::new();
        n.read_to_string(&mut buf).unwrap();

        let mut n = OpenOptions::new()
            .write(true)
            .create(false)
            .open("rayout/.n.txt")
            .unwrap();

        n.write_all(
            (buf.trim().parse::<i32>().unwrap() + 1)
                .to_string()
                .as_bytes(),
        )
        .unwrap();
    }
    let filename = format!("rayout/trace-{}.ppm", buf.trim());
    let mut file = fs::File::create(filename.clone()).unwrap();
    file.write_all(format!("P3\n{IMG_W} {IMG_H}\n255\n").as_bytes())
        .unwrap();

    for thread_vec in data.iter().rev() {
        for bytes in thread_vec.iter().rev() {
            file.write_all(&bytes).unwrap();
        }
    }
    println!("wrote to {filename}, exiting.")
}
