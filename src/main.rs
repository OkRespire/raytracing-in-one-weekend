mod hit;
mod interval;
mod ray;
mod shape;

use hit::Hittable;
use interval::Interval;
use nalgebra::Vector3;
use shape::Sphere;
use std::{
    io::{self, Write},
    sync::Arc,
};

type Colour = Vector3<f64>;

const PI: f64 = 3.141_592_653_589_793;
const INFINITY: f64 = f64::INFINITY;

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    //calculates image to ensure it is greater than 1
    let mut height = (width as f64 / aspect_ratio) as i32;
    height = if height < 1 { 1 } else { height };

    let focal_len = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let cam_center = Vector3::new(0.0, 0.0, 0.0);

    //calculate the vectors across the horizontal and down the vertical edges
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    //calculate the delta vectors horizontally and vertically
    let pixel_d_u = Vector3::new(viewport_u.x / width as f64, viewport_u.y, viewport_u.z);
    let pixel_d_v = Vector3::new(viewport_v.x, viewport_v.y / height as f64, viewport_v.z);

    let viewport_upper_left =
        cam_center - Vector3::new(0.0, 0.0, focal_len) - (viewport_u / 2.0) - (viewport_v / 2.0);

    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_d_v + pixel_d_u);

    let mut world = hit::HitList::new();

    world.add(Arc::new(Sphere {
        centre: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    world.add(Arc::new(Sphere {
        centre: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    //Render
    println!("P3\n{} {}\n255", width, height);

    for j in 0..height {
        eprint!("\rScanlines remaining {} ", (height - j));
        io::stderr().flush().unwrap();

        for i in 0..width {
            let pixel_center = pixel_00_loc + (i as f64 * pixel_d_u) + (j as f64 * pixel_d_v);

            let ray_dir = pixel_center - cam_center;
            let r = ray::Ray::new(cam_center, ray_dir);

            let pixel_col = ray_colour(&r, &world);

            write_colour(&pixel_col);
        }
    }
    eprintln!("\rDone               ")
}

fn ray_colour<T: Hittable>(r: &ray::Ray, world: &T) -> Colour {
    if let Some(hit_rec) = world.hit(r, Interval::new(0.0, INFINITY)) {
        return 0.5 * (hit_rec.normal + Colour::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction();
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
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
