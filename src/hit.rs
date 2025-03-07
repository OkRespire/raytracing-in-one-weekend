use std::sync::Arc;

use crate::{interval::Interval, ray::Ray};
use nalgebra::Vector3;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub front_face: bool,
}

pub struct HitList {
    list: Vec<Arc<dyn Hittable>>, //added for multithreading support
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vector3<f64>) {
        // Sets the normal vector for hit record
        // NOTE: outward_norm assumes to have the unit length

        let front_face = r.direction().dot(&outward_norm) < 0.0;
        self.normal = if front_face {
            outward_norm //outside sphere
        } else {
            -outward_norm //inside sphere
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: 0.0,
            p: Vector3::zeros(),
            normal: Vector3::zeros(),
            front_face: false,
        }
    }
}

impl HitList {
    pub fn new() -> Self {
        HitList { list: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.list.push(obj);
    }
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for item in &self.list {
            if let Some(hit) = item.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                hit_anything = true;
                closest_so_far = hit.t;
                temp_rec = hit;
            }
        }

        if hit_anything {
            Some(temp_rec) // Return the closest hit
        } else {
            None // No hit
        }
    }
}
