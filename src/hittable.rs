pub mod hittable {
    use crate::{vector::vector::{Point3, Vec3}, ray::ray::Ray};


    pub struct HitRecord {
        pub p: Point3,
        pub normal: Vec3,
        pub t: f64
    }

    pub trait Hittable {
        fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    }
}