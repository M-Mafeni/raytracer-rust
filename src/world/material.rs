use crate::{ray::{Ray, ray}, vector::{Color, Point3, random::random_unit_vector, utility::{reflect, dot_product}}};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {albedo: Color},
    Metal {albedo: Color},
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
                let x = normal + random_unit_vector();
                // catch degenerate scatter direction
                let scatter_direction = if x.near_zero() {normal} else {x};
                let scattered = ray(p, scatter_direction);
                Some(ScatterResult { attenuation: *albedo, scattered })
            },
            Material::Metal { albedo } => {
                let reflected = reflect(r.direction().unit_vector(), normal);
                let scattered = ray(p, reflected);
                if dot_product(scattered.direction(), normal) > 0.0 {
                    Some(ScatterResult {attenuation: *albedo, scattered})
                } else {
                    None
                }
            }
        }
    }
}
