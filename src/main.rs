use std::fs::File;
use std::io::{Write};

use crate::vector::vector::{Vec3, dot_product, cross_product};
use crate::writer::writer::write_color;

mod vector;
mod writer;
fn main() -> std::io::Result<()>{
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;

    let v1 = Vec3::new(4.0, 5.0, 6.0);
    let v2 = Vec3::new(8.0, 7.0, 10.0);

    println!("v1 + v2 is {:?}", v1 + v2);
    println!("v1 - v2 is {:?}", v1 - v2);
    println!("v2 - v1 is {:?}", v2 - v1);
    println!("v1 * v2 is {:?}", v1 * v2);
    println!("v1 * 5 is {:?}", v1 * 5.0);
    println!("5 * v1 is {:?}", 5.0 * v1);
    println!("length of v1 is {}", v1.length());
    println!("unit vector of v1 is {:?}", v1.unit_vector());
    println!("v1.v2 is {:?}", dot_product(v1, v2));
    println!("v1 X v2 is {:?}", cross_product(v1, v2));

    // Create Image PPM
    let buffer = File::create("image.ppm")?;
    writeln!(&buffer, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            write_color(&buffer,
                Vec3::new(
                    i as f64 / (IMAGE_WIDTH - 1) as f64,
                    j as f64 / (IMAGE_HEIGHT - 1) as f64,
                    0.25))?
        }
    }
    eprintln!("\nDone.");
    Ok(())
    
}
