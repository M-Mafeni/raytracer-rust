use crate::utility::random::{random_double, random_double_range};

use super::{Vec3, vec3, zero_vector, utility::{dot_product, unit_vector}};

pub fn random() -> Vec3 {
    vec3(random_double(), random_double(), random_double())
}

pub fn random_range(min: f64, max: f64) -> Vec3 {
    vec3(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
}

pub fn random_in_unit_sphere() -> Vec3 {
    while true {
        let p = random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
    zero_vector()
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot_product(in_unit_sphere, normal) > 0.0 { // In the same hemisphere as normal
        in_unit_sphere
    } else {
        -1.0 * in_unit_sphere
    }
}