use crate::{ray::Ray, vector::{Color, Point3}};

pub mod lambertian;
pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, r: &Ray, p: Point3, normal: Point3, t: f64) -> Option<ScatterResult>;
}
