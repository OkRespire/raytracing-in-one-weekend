use crate::{
    hit::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};
use nalgebra::Vector3;

pub struct Sphere {
    pub radius: f64,
    pub centre: Vector3<f64>,
}

enum Shape {
    Sphere(Sphere),
    //Plane(Plane),
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.centre - r.origin();
        let a = r.direction().magnitude_squared();
        let h = r.direction().dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None; //no real roots
        }

        let sqrtd = discriminant.sqrt();

        //find nearest root that is in the range that is accepted
        let mut root = (h - sqrtd) / a;

        if root <= ray_t.min || root >= ray_t.max {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || root >= ray_t.max {
                return None; //No valid hit
            }
        }

        let p = r.at(root);
        let outward_norm = (p - self.centre) / self.radius;
        let mut hit_rec = HitRecord {
            t: root,
            p,
            normal: outward_norm,
            front_face: false,
        };

        hit_rec.set_face_normal(r, outward_norm);

        Some(hit_rec)
    }
}
