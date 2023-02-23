use crate::{ray::{Ray, ray}, vector::{Color, Point3, random::{random_unit_vector, random_in_unit_sphere}, utility::{reflect, dot_product, refract}, color}};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {albedo: Color},
    Metal {albedo: Color, fuzz: f64},
    Dielectric {refraction_index: f64}
}

pub fn metal(albedo: Color, f: f64) -> Material {
    let fuzz = if f < 1.0 {f} else {1.0};
    Material::Metal { albedo, fuzz }
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray
}

impl Material {
    pub fn scatter(&self, r: &Ray, p: Point3, normal: Point3, front_face: bool, t: f64) -> Option<ScatterResult> {
        match self {
            Material::Lambertian {albedo} => {
                let x = normal + random_unit_vector();
                // catch degenerate scatter direction
                let scatter_direction = if x.near_zero() {normal} else {x};
                let scattered = ray(p, scatter_direction);
                Some(ScatterResult { attenuation: *albedo, scattered })
            },
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(r.direction().unit_vector(), normal);
                let scattered = ray(p, reflected + *fuzz * random_in_unit_sphere());
                if dot_product(scattered.direction(), normal) > 0.0 {
                    Some(ScatterResult {attenuation: *albedo, scattered})
                } else {
                    None
                }
            },
            Material::Dielectric { refraction_index } => {
                let refraction_ratio = if front_face {1.0/refraction_index} else {*refraction_index};
                let unit_direction = r.direction().unit_vector();

                let cos_theta = dot_product(-unit_direction, normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract {
                    reflect(unit_direction, normal)
                } else {
                    refract(unit_direction, normal, refraction_ratio)
                };

                let attenuation = color(1.0, 1.0, 1.0);
                let scattered = ray(p, direction);
                Some(ScatterResult {attenuation, scattered})
            }
        }
    }
}
