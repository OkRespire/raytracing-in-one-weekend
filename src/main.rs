mod camera;
mod hit;
mod interval;
mod ray;
mod shape;

use camera::Camera;
use nalgebra::Vector3;
use shape::Sphere;
use std::sync::Arc;

type Colour = Vector3<f64>;

const PI: f64 = 3.141_592_653_589_793;

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

    let cam = Camera::new(aspect_ratio, img_width);

    cam.render(&world);
}

fn write_colour(pixel_colour: &Colour) {
    let r = pixel_colour.x;
    let g = pixel_colour.y;
    let b = pixel_colour.z;

    let rbyte = (255.999 * r) as i64;
    let gbyte = (255.999 * g) as i64;
    let bbyte = (255.999 * b) as i64;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}
