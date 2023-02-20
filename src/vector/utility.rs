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