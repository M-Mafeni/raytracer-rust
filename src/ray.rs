pub mod ray {
    use crate::vector::vector::{Point3, Vec3};

    pub struct Ray {
        origin: Point3,
        direction: Vec3
    }

    impl Ray {
        pub fn origin(&self) -> Vec3 {
            self.origin
        }

        pub fn direction(&self) -> Vec3 {
            self.direction
        }

        pub fn at(&self, t: f64) -> Point3 {
            self.origin + t * self.direction
        }
    }
}