use crate::{ray::{Ray, ray}, vector::{Color, Point3, random_unit_vector}};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {albedo: Color}
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Scatter {
    fn scatter(&self, r: &Ray, p: Point3, normal: Point3, t: f64) -> Option<ScatterResult>;
}

impl Scatter for Material {
    fn scatter(&self, r: &Ray, p: Point3, normal: Point3, t: f64) -> Option<ScatterResult> {
        match self {
            Material::Lambertian {albedo} => {
                let scatter_direction = normal + random_unit_vector();
                let scattered = ray(p, scatter_direction);
                Some(ScatterResult { attenuation: *albedo, scattered })
            }
        }
    }
}
