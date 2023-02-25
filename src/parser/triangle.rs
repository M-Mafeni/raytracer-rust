use std::collections::HashMap;
use std::fs;

use crate::{world::{shapes::triangle::{Triangle, triangle}, material::Material}, vector::{Point3, point3, zero_vector}};

fn get_points(content: &String) -> Vec<Point3> {
    let to_point = |line: &str| -> Point3 {
        let points: Vec<f64> = line.split(' ').skip(1).map(|x| x.parse().unwrap()).collect();
        point3(points[0], points[1], points[2])
    };

    content.lines().filter(|x| {x.starts_with("v")}).map(to_point).collect()
}

pub fn parse_triangles(file_path: &str, material_map: HashMap<String, Material> ) -> Vec<Triangle> {

    let content = fs::read_to_string(file_path)
        .expect("Failed to read file");
    let points = get_points(&content);
    let mut material = &Material::Lambertian { albedo: zero_vector() };

    let mut triangles = Vec::new();
    for line in content.lines().filter(|x| x.starts_with("usemtl") || x.starts_with("f") ) {
        if line.starts_with("usemtl") {
            let name = line.split(' ').nth(1).unwrap();
            material = material_map.get(name).unwrap();
        } else {
            // Make triangle
            let indexes: Vec<usize> = line.split(' ').skip(1).map(|x| {
                let mut s = x.to_string();
                s.pop();
                s.parse::<usize>().unwrap() - 1
            }).collect();
            let t = triangle(
                points[indexes[0]],
                points[indexes[1]],
                points[indexes[2]],
                *material
            );
            triangles.push(t);
        }
    }

    triangles
}