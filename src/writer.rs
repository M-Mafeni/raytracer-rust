pub mod writer {
    use crate::vector::vector::Color;

    use std::io::{Result, Write};

    pub fn write_color<W: Write>(mut out: W, pixel_color: Color) -> Result<()> {
        writeln!(
            out,
            "{} {} {}",
            (255.999 * pixel_color.x()) as u32,
            (255.999 * pixel_color.y()) as u32,
            (255.999 * pixel_color.z()) as u32
        )
    }

}