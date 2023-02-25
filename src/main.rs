use std::f64::INFINITY;
use std::fs::File;
use std::io::{Write};
use std::time::Instant;

use hittable::Hittable;
use utility::random::random_double_range;
use vector::{Color};
use world::material::ScatterResult;
use world::shapes::sphere;

use crate::camera::{cam};
use crate::hittable::{HittableList, create_new_hittable_list};
use crate::ray::{Ray};
use crate::world::material::{Material, metal};
use crate::world::shapes::sphere::sphere;
use crate::utility::random::random_double;
use crate::vector::{color, point3, vec3};
use crate::writer::write_color;

mod vector;
mod writer;
mod ray;
mod hittable;
mod world;
mod utility;
mod camera;


fn random_scene() -> HittableList {
    let mut world: HittableList = create_new_hittable_list();

    let ground_material = Material::Lambertian {albedo: color(0.5, 0.5, 0.5)};
    world.add_new_hittable(sphere(point3(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center =  point3(
                f64::from(a) + 0.9 * random_double(),
                0.2,
                f64::from(b) + 0.9 * random_double()
            );

            if (center - point3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vector::random::random() * vector::random::random();
                    let material = Material::Lambertian { albedo };
                    world.add_new_hittable(sphere(center, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vector::random::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let material = metal(albedo, fuzz);
                    world.add_new_hittable(sphere(center, 0.2, material));
                } else {
                    // glass
                    let material = Material::Dielectric { refraction_index: 1.5 };
                    world.add_new_hittable(sphere(center, 0.2, material));
                }
            }
        }
    }

    let material1 =  Material::Dielectric { refraction_index: 1.5 };
    world.add_new_hittable(sphere(point3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian { albedo: color(0.4, 0.2, 0.1) };
    world.add_new_hittable(sphere(point3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = metal(color(0.7, 0.6, 0.5), 0.0);
    world.add_new_hittable(sphere(point3(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn ray_color(r: Ray, world: &HittableList, depth: u8) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(&r, 0.001, INFINITY) {
        let scatter_result = hit_record.material.scatter(&r, hit_record.p, hit_record.normal, hit_record.front_face, hit_record.t);
        match scatter_result {
            Some(ScatterResult {attenuation, scattered}) => return attenuation * ray_color(scattered, world, depth - 1),
            None => return color(0.0, 0.0, 0.0) 
        }
    }
    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()>{
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u16 = 1200;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO as f32) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    // let material_ground =  Material::Lambertian { albedo: color(0.8, 0.8, 0.0) };
    // let material_center =  Material::Lambertian { albedo: color(0.1, 0.2, 0.5) };
    // let material_left =  Material::Dielectric { refraction_index: 1.5 };
    // let material_right =  metal(color(0.8, 0.6, 0.2), 0.0);

    // let mut world: HittableList = create_new_hittable_list();
    // world.add_new_hittable(sphere(point3(0.0, -100.5, -1.0), 100.0, material_ground));
    // world.add_new_hittable(sphere(point3(0.0, 0.0, -1.0), 0.5, material_center));
    // world.add_new_hittable(sphere(point3(-1.0, 0.0, -1.0), 0.5, material_left));
    // world.add_new_hittable(sphere(point3(-1.0, 0.0, -1.0), -0.45, material_left));
    // world.add_new_hittable(sphere(point3(1.0, 0.0, -1.0), 0.5, material_right));
    let world = random_scene();


    // Camera
    let look_from = point3(13.0, 2.0, 3.0);
    let look_at = point3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = cam(look_from, look_at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    let now = Instant::now();
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
    eprintln!("\nTime taken {:?}", Instant::now() - now);
    eprintln!("\nDone.");
    Ok(())
    
}
