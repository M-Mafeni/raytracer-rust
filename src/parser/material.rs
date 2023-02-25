use std::collections::HashMap;
use std::fs;


use crate::{world::material::{Material}, vector::color};

fn parse_material(text: &str) -> (String, Material) {
    let mut lines =  text.lines();
    let name_line = lines.next().unwrap();
    let name = name_line.split(' ').nth(1).unwrap().to_string();

    let colour_line = lines.next().unwrap();
    let colour_vals: Vec<f64> = colour_line
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    let albedo = color(colour_vals[0], colour_vals[1], colour_vals[2]);
    let material = Material::Lambertian { albedo };
    (name, material)
}

pub fn parse_materials(file_path: &str) -> HashMap<String, Material> {
    let mut hashmap = HashMap::new();
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");
    contents.split("\r\n\r\n").for_each(|group| {
        let (name, material) = parse_material(group);
        hashmap.insert(name, material);
    });
    hashmap
}