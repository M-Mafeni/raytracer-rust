use crate::vector::vector::{Vec3, dot_product, cross_product};

mod vector;
fn main() {
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


    /* Uncomment for creating ppm image
    
        println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
        for j in (0..IMAGE_HEIGHT).rev() {
            eprint!("\rScanlines Remaining: {} ", j);
            for i in 0..IMAGE_WIDTH {
                let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
                let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
                let b: f32 = 0.25;

                let ir: u16 = (255.999 * r) as u16;
                let ig: u16 = (255.999 * g) as u16;
                let ib: u16 = (255.999 * b) as u16;
                println!("{} {} {}", ir, ig, ib);
            }
        }
        eprintln!("\nDone.");
    
     */
}
