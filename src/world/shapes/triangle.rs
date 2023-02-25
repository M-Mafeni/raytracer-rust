use crate::{vector::Point3, world::material::Material, hittable::Hittable};

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
        None
    }
}