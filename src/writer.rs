pub mod writer {
    use crate::{vector::vector::Color, utility::math::math::clamp};

    use std::io::{Result, Write};

    pub fn write_color<W: Write>(mut out: W, pixel_color: Color, samples_per_pixel: u16) -> Result<()> {

        let scale = 1.0 / f64::from(samples_per_pixel);
        let r = pixel_color.x() * scale;
        let g = pixel_color.y() * scale;
        let b = pixel_color.z() * scale;

        writeln!(
            out,
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as u32,
            (256.0 * clamp(g, 0.0, 0.999)) as u32,
            (256.0 * clamp(b, 0.0, 0.999)) as u32
        )
    }

}