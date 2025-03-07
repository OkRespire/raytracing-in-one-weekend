use std::io::{self, Write};

use nalgebra::Vector3;

use crate::{hit::Hittable, interval::Interval, ray, write_colour};

pub struct Camera {
    pub aspect_ratio: f64,      //ratio of img width/ img img_height
    pub img_width: i64,         //rendered img width
    img_height: i64,            //rendered img height
    center: Vector3<f64>,       //location of the centre of the canmera
    pixel_00_loc: Vector3<f64>, //location of 0,0
    pixel_du: Vector3<f64>,     //offset of the pixel to the right
    pixel_dv: Vector3<f64>,     //offset of the pixel below
}
type Colour = Vector3<f64>;

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: i64) -> Self {
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

        Camera {
            aspect_ratio,
            img_width,
            img_height,
            center,
            pixel_00_loc,
            pixel_du,
            pixel_dv,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        //Render
        println!("P3\n{} {}\n255", self.img_width, self.img_height);

        for j in 0..self.img_height {
            eprint!("\rScanlines remaining {} ", (self.img_height - j));
            io::stderr().flush().unwrap();

            for i in 0..self.img_width {
                let pixel_center =
                    self.pixel_00_loc + (i as f64 * self.pixel_du) + (j as f64 * self.pixel_dv);

                let ray_dir = pixel_center - self.center;
                let r = ray::Ray::new(self.center, ray_dir);

                let pixel_col = Self::ray_colour(&r, world);

                write_colour(&pixel_col);
            }
        }
        eprintln!("\rDone               ");
    }

    fn ray_colour<T: Hittable>(r: &ray::Ray, world: &T) -> Colour {
        if let Some(hit_rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (hit_rec.normal + Colour::new(1.0, 1.0, 1.0));
        }
        let unit_direction = r.direction();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }
}
