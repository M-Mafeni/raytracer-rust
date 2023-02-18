use std::fs::File;
use std::io::{Write};

use vector::vector::{Color, Point3, unit_vector};

use crate::ray::ray::{Ray, ray};
use crate::vector::vector::{Vec3, dot_product, vec3, zero_vector};
use crate::writer::writer::write_color;

mod vector;
mod writer;
mod ray;
mod hittable;
mod shapes;
mod math;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot_product(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b*half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Color {
    let hit_value = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if hit_value > 0.0 {
        let n = unit_vector(ray.at(hit_value) - vec3(0.0, 0.0, -1.0));
        return 0.5 * vec3(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    }
    let unit_dir = ray.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()>{
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO as f32) as u16;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = zero_vector();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Create Image PPM
    let buffer = File::create("image.ppm")?;
    writeln!(&buffer, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let v = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let r = ray(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            write_color(&buffer, pixel_color)?
        }
    }
    eprintln!("\nDone.");
    Ok(())
    
}
