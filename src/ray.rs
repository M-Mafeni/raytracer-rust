pub mod ray {
    use crate::vector::{Point3, Vec3};

    pub fn ray(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction}
    }
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