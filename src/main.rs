mod ray;
mod shape;

use nalgebra::Vector3;
use std::io::{self, Write};

type Colour = Vector3<f64>;

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

    //Render
    println!("P3\n{} {}\n255", width, height);

    for j in 0..height {
        eprint!("\rScanlines remaining {} ", (height - j));
        io::stderr().flush().unwrap();

        for i in 0..width {
            let pixel_center = pixel_00_loc + (i as f64 * pixel_d_u) + (j as f64 * pixel_d_v);

            let ray_dir = pixel_center - cam_center;
            let r = ray::Ray::new(cam_center, ray_dir);

            let pixel_col = ray_colour(&r);

            write_colour(&pixel_col);
        }
    }
    eprintln!("\rDone               ")
}

fn ray_colour(r: &ray::Ray) -> Colour {
    if hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Colour::new(1.0, 0.0, 0.0);
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

fn hit_sphere(centre: &Vector3<f64>, radius: f64, r: &ray::Ray) -> bool {
    let oc = centre - r.origin();
    let oc: Vector3<f64> = oc.into();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}
