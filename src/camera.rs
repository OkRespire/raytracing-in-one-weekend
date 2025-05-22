use std::io::{self, Write};

use nalgebra::Vector3;

use crate::{
    hit::Hittable, interval::Interval, rand_f64, rand_on_hemisphere, rand_unit_vec3, ray,
    write_colour,
};

pub struct Camera {
    pub aspect_ratio: f64,      //ratio of img width/ img img_height
    pub img_width: i64,         //rendered img width
    pub samples_per_pixel: i64, //Count of random samples per pixel
    pub max_depth: i64,         // Max number of ray bounces in a scene
    img_height: i64,            //rendered img height
    center: Vector3<f64>,       //location of the centre of the canmera
    pixel_00_loc: Vector3<f64>, //location of 0,0
    pixel_du: Vector3<f64>,     //offset of the pixel to the right
    pixel_dv: Vector3<f64>,     //offset of the pixel below
    pixel_samples_scale: f64,   //Colou
}
type Colour = Vector3<f64>;

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: i64, samples_per_pixel: i64, max_depth: i64) -> Self {
        //Image
        //calculates image to ensure it is greater than 1
        let mut img_height = (img_width as f64 / aspect_ratio) as i64;
        img_height = if img_height < 1 { 1 } else { img_height };

        let focal_len = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
        let center = Vector3::new(0.0, 0.0, 0.0);

        //calculate the vectors across the horizontal and down the vertical edges
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        //calculate the delta vectors horizontally and vertically
        let pixel_du = viewport_u.scale(1.0 / img_width as f64);
        let pixel_dv = viewport_v.scale(1.0 / img_height as f64);

        let viewport_upper_left = center
            - Vector3::new(0.0, 0.0, focal_len)
            - viewport_u.scale(0.5)
            - viewport_v.scale(0.5);

        let pixel_00_loc = viewport_upper_left + (pixel_dv + pixel_du) * 0.5;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            max_depth,
            img_height,
            center,
            pixel_00_loc,
            pixel_du,
            pixel_dv,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        //Render
        println!("P3\n{} {}\n255", self.img_width, self.img_height);

        for j in 0..self.img_height {
            eprint!("\rScanlines remaining {} ", (self.img_height - j));
            io::stderr().flush().unwrap();

            let mut pixel_col = Colour::zeros();
            for i in 0..self.img_width {
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_col += self.ray_colour(&r, self.max_depth, world);
                }
                pixel_col *= self.pixel_samples_scale;

                write_colour(&pixel_col);
            }
        }
        eprintln!("\rDone               ");
    }

    fn get_ray(&self, i: i64, j: i64) -> ray::Ray {
        let offset = self.sample_square();
        let pix_sample = self.pixel_00_loc
            + (i as f64 + offset.x) * self.pixel_du
            + (j as f64 + offset.y) * self.pixel_dv;

        let ray_orig = self.center;
        let ray_dir = pix_sample - ray_orig;

        ray::Ray::new(ray_orig, ray_dir)
    }

    fn sample_square(&self) -> Vector3<f64> {
        Vector3::new(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
    }

    fn ray_colour<T: Hittable>(&self, r: &ray::Ray, depth: i64, world: &T) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let dir = hit_rec.normal + rand_unit_vec3();

            return 0.9 * self.ray_colour(&ray::Ray::new(hit_rec.p, dir), depth - 1, world);
        }
        let unit_direction = r.direction();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }
}
