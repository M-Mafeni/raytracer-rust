use crate::{vector::vector::{Point3, dot_product}, hittable::hittable::{Hittable, HitRecord}};


pub fn sphere(center: Point3, radius: f64) -> Sphere {
    Sphere {
        center,
        radius
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let radius = self.radius;
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot_product(oc, r.direction());
        let c = oc.length_squared() - radius * radius;

        let discriminant = half_b*half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / radius;
        Some(HitRecord{t, p, normal})
    }
}


