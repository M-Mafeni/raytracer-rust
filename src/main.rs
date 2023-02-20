use std::f64::INFINITY;
use std::fs::File;
use std::io::{Write};

use hittable::Hittable;
use vector::{Color};
use world::material::{Scatter, ScatterResult};

use crate::camera::initialise_camera;
use crate::hittable::{HittableList, create_new_hittable_list};
use crate::ray::{Ray};
use crate::world::material::Material;
use crate::world::shapes::sphere::sphere;
use crate::utility::random::random_double;
use crate::vector::{Vec3, vec3, color};
use crate::writer::write_color;

mod vector;
mod writer;
mod ray;
mod hittable;
mod world;
mod utility;
mod camera;

fn ray_color(r: Ray, world: &HittableList, depth: u8) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(&r, 0.001, INFINITY) {
        let scatter_result = hit_record.material.scatter(&r, hit_record.p, hit_record.normal, hit_record.t);
        match scatter_result {
            Some(ScatterResult {attenuation, scattered}) => return attenuation * ray_color(scattered, world, depth - 1),
            None => return color(0.0, 0.0, 0.0) 
        }
    }
    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()>{
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO as f32) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world: HittableList = create_new_hittable_list();
    let lambertian = Material::Lambertian { albedo: vec3(0.5, 0.5, 0.5) };
    world.add_new_hittable(sphere(vec3(0.0, 0.0, -1.0), 0.5, lambertian));
    world.add_new_hittable(sphere(vec3(0.0, -100.5, -1.0), 100.0, lambertian));

    // Camera
    let camera = initialise_camera();

    // Create Image PPM
    let buffer = File::create("image.ppm")?;
    writeln!(&buffer, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = color(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (f64::from(i) + random_double()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + random_double()) / f64::from(IMAGE_HEIGHT - 1);
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_color(&buffer, pixel_color, samples_per_pixel)?
        }
    }
    eprintln!("\nDone.");
    Ok(())
    
}
