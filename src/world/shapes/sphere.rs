use crate::{vector::{Point3, utility::dot_product}, hittable::{Hittable, HitRecord}, world::material::Material};


pub fn sphere(center: Point3, radius: f64, material: Material) -> Sphere {
    Sphere {
        center,
        radius,
        material
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let outward_normal = (p - self.center) / radius;
        // Set whether it faces backwards or forwards
        let front_face = dot_product(r.direction(), outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            _ => -1.0 * outward_normal
        };
        Some(HitRecord{t, p, normal, front_face, material: self.material})
    }
}


