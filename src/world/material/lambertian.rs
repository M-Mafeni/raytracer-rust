use crate::{vector::{Color, random_unit_vector}, ray::ray};

use super::Material;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color
}

pub fn lambertian(albedo: Color) -> Lambertian {
    Lambertian { albedo }
}

impl Material for Lambertian {
    fn scatter(&self, r: &crate::ray::Ray, p: crate::vector::Point3, normal: crate::vector::Point3, t: f64) -> Option<super::ScatterResult> {
        let scatter_direction = normal + random_unit_vector();
        let scattered = ray(p, scatter_direction);
        Some(super::ScatterResult { attenuation: self.albedo, scattered })
    }
}
