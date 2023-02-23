use super::Vec3;

pub fn unit_vector(v: Vec3) -> Vec3 {
    v.scale(1.0 / v.length())
}

pub fn dot_product(u: Vec3, v: Vec3) -> f64 {
    let Vec3{points: p1} = u;
    let Vec3{points: p2} = v;
    p1[0] * p2[0] + p1[1] * p2[1] + p1[2] * p2[2]
}

pub fn cross_product(u: Vec3, v: Vec3) -> Vec3 {
    let Vec3{points: p1} = u;
    let Vec3{points: p2} = v;
    Vec3 {
        points: [
            p1[1] * p2[2] - p1[2] * p2[1],
            p1[2] * p2[0] - p1[0] * p2[2],
            p1[0] * p2[1] - p1[1] * p2[0]
        ]
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot_product(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot_product(-uv, n).min(1.0);
    let r_out_perpendicular = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt()) * n;
    r_out_perpendicular + r_out_parallel
}