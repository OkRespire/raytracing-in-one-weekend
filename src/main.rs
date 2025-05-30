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
    let max_depth = 50;

    let cam = Camera::new(aspect_ratio, img_width, samples_per_pix, max_depth);
    cam.render(&world);
}

//Helper functions

fn linear_to_gamma(lc: f64) -> f64 {
    if lc > 0.0 {
        return lc.sqrt();
    }
    0.0
}

fn write_colour(pixel_colour: &Colour) {
    // // Print the raw colour values for debugging
    // eprintln!(
    //     "Raw Colour: ({}, {}, {})",
    //     pixel_colour.x, pixel_colour.y, pixel_colour.z
    // );

    // Define an interval for clamping colour values to [0.0, 1.0]
    let intensity = interval::Interval::new(0.0, 1.0);

    // Clamp the colour values using the interval
    let r = linear_to_gamma(intensity.clamp(pixel_colour.x));
    let g = linear_to_gamma(intensity.clamp(pixel_colour.y));
    let b = linear_to_gamma(intensity.clamp(pixel_colour.z));

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

/**
 Makes a random float64 value
*/
fn rand_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>()
}

fn rand_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}

fn rand_vec3() -> Vector3<f64> {
    Vector3::new(rand_f64(), rand_f64(), rand_f64())
}

fn rand_vec3_range(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        rand_f64_range(min, max),
        rand_f64_range(min, max),
        rand_f64_range(min, max),
    )
}

fn rand_unit_vec3() -> Vector3<f64> {
    loop {
        let p = rand_vec3_range(-1.0, 1.0);
        let len_sq = p.magnitude_squared();
        if len_sq <= 1.0 && 1e-160 < len_sq {
            return p / len_sq.sqrt();
        }
    }
}

fn rand_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = rand_unit_vec3();
    if on_unit_sphere.dot(&normal) > 0.0 {
        return on_unit_sphere;
    }

    -on_unit_sphere
}
