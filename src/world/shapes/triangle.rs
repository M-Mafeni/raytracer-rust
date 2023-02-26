use crate::{vector::{Point3, utility::{cross_product, dot_product}}, world::material::Material, hittable::{Hittable, HitRecord}};

#[derive(Debug)]
pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    material: Material
}

pub fn triangle(a: Point3, b: Point3, c: Point3, material: Material) -> Triangle {
    Triangle { a, b, c, material }
}

impl Hittable for Triangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        // TODO Implement
        let Triangle { a: v0, b: v1, c: v2, material } = *self;

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;

        let normal = cross_product(v0v1, v0v2).unit_vector();
        let denom = dot_product(normal, r.direction());
        if denom == 0.0 {
            return None;
        }
        let d = dot_product(normal, v0);
        let t = (d - dot_product(normal, r.origin())) / denom;
        if t < t_min || t > t_max {
            return None;
        }
        let p = r.at(t);

        // edge 0
        let e0 = v1 - v0;
        let c0 = p - v0;
        if dot_product(normal, cross_product(e0, c0)) < 0.0 {
            return None;
        }

        // edge 1
        let e1 = v2 - v1;
        let c1 = p - v1;
        if dot_product(normal, cross_product(e1, c1)) < 0.0 {
            return None;
        }

        let e2 = v0 - v2;
        let c2 = p - v2;
        if dot_product(normal, cross_product(e2, c2)) < 0.0 {
            return None;
        }
        Some(HitRecord {p, normal, t, front_face: dot_product(normal, r.direction()) > 0.0, material})
    }
}