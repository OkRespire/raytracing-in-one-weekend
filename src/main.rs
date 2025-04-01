mod camera;
mod hit;
mod interval;
mod ray;
mod shape;

use camera::Camera;
use nalgebra::Vector3;
use rand::Rng;
use shape::Sphere;
use std::sync::Arc;

type Colour = Vector3<f64>;

const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() {
    let mut world = hit::HitList::new();
    world.add(Arc::new(Sphere {
        centre: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    world.add(Arc::new(Sphere {
        centre: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let samples_per_pix = 10;

    let cam = Camera::new(aspect_ratio, img_width, samples_per_pix);
    cam.render(&world);
}

//Helper functions

fn write_colour(pixel_colour: &Colour) {
    // // Print the raw colour values for debugging
    // eprintln!(
    //     "Raw Colour: ({}, {}, {})",
    //     pixel_colour.x, pixel_colour.y, pixel_colour.z
    // );

    // Define an interval for clamping colour values to [0.0, 1.0]
    let intensity = interval::Interval::new(0.0, 1.0);

    // Clamp the colour values using the interval
    let r = intensity.clamp(pixel_colour.x);
    let g = intensity.clamp(pixel_colour.y);
    let b = intensity.clamp(pixel_colour.z);

    // // Print the clamped colour values for debugging
    // eprintln!("Clamped Colour: ({}, {}, {})", r, g, b);

    // Convert the clamped values to 8-bit colour values
    let rbyte = (255.999 * r) as i64;
    let gbyte = (255.999 * g) as i64;
    let bbyte = (255.999 * b) as i64;

    // Print the final colour values in PPM format
    println!("{} {} {}", rbyte, gbyte, bbyte);
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

fn rand_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>()
}

fn rand_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}
