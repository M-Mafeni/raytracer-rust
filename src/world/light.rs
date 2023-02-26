use crate::vector::{Point3, Color};

pub struct Light {
    pub position: Point3,
    pub colour: Color
}

pub fn light(position: Point3, colour: Color) -> Light {
    Light { position, colour}
}