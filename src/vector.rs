pub mod vector {
    use std::ops;

    use crate::utility::random::random::{random_double, random_double_range};

    pub fn zero_vector() -> Vec3 {
        Vec3 {
            points: [0.0, 0.0, 0.0]
        }
    }

    pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            points: [x, y, z]
        }
    }

    pub fn random() -> Vec3 {
        vec3(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        vec3(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        while true {
            let p = random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
        zero_vector()
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = random_in_unit_sphere();
        if dot_product(in_unit_sphere, normal) > 0.0 { // In the same hemisphere as normal
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }

    pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
        Vec3 {
            points: [x, y, z]
        }
    }

    pub fn color(x: f64, y: f64, z: f64) -> Color {
        Vec3 {
            points: [x, y, z]
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v.scale(1.0 / v.length())
    }


    #[derive(Debug, Clone, Copy)]
    pub struct Vec3 {
        points: [f64; 3]
    }

    impl Vec3 {

        pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
            Vec3 {
                points: [e0, e1, e2]
            }
        }

        pub fn x(&self) -> f64 {
            self.points[0]
        }

        pub fn y(&self) -> f64 {
            self.points[1]
        }

        pub fn z(&self) -> f64 {
            self.points[2]
        }

        fn scale(&self, t: f64) -> Vec3 {
            let Vec3 { points: p } = self;
            Vec3 { points: [p[0] * t, p[1] * t, p[2] * t] }
        }

        pub fn length_squared(&self) -> f64 {
            let Vec3 { points } = self;
            points[0] * points[0] + points[1] * points[1] + points[2] * points[2]
        }

        pub fn length(&self) -> f64 {
            let t = self.length_squared();
            t.sqrt()
        }

        pub fn unit_vector(&self) -> Vec3 {
            self.scale(1.0 / self.length())
        }
    }

    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;
        fn add(self, rhs: Vec3) -> Self::Output {
            let Vec3{points: p1} = self;
            let Vec3{points: p2} = rhs;
            Vec3 {
                points: [
                    p1[0] + p2[0],
                    p1[1] + p2[1],
                    p1[2] + p2[2],
                ]
            }
        }
    }

    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;
        fn sub(self, rhs: Vec3) -> Self::Output {
            let Vec3{points: p1} = self;
            let Vec3{points: p2} = rhs;
            Vec3 {
                points: [
                    p1[0] - p2[0],
                    p1[1] - p2[1],
                    p1[2] - p2[2],
                ]
            }
        }
    }

    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;
        fn mul(self, rhs: Vec3) -> Self::Output {
            let Vec3{points: p1} = self;
            let Vec3{points: p2} = rhs;
            Vec3 {
                points: [
                    p1[0] * p2[0],
                    p1[1] * p2[1],
                    p1[2] * p2[2],
                ]
            }
        }
    }

    impl ops::Mul<f64> for Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: f64) -> Self::Output {
            self.scale(rhs)
        }
    }

    impl ops::Mul<Vec3> for f64 {
        type Output = Vec3;

        fn mul(self, rhs: Vec3) -> Self::Output {
            rhs.scale(self)
        }
    }

    impl ops::Div<f64> for Vec3 {
        type Output = Vec3;

        fn div(self, rhs: f64) -> Self::Output {
            self.scale(1.0 / rhs)
        }
    }

    pub type Point3 = Vec3;
    pub type Color = Vec3;

    // utility functions

    pub fn dot_product(u: Vec3, v: Vec3) -> f64 {
        let Vec3{points: p1} = u;
        let Vec3{points: p2} = v;
        p1[0] * p2[0] + p1[1] * p2[1] + p1[2] * p2[2]
    }

    pub fn cross_product(u: Vec3, v: Vec3) -> Vec3 {
        let Vec3{points: p1} = u;
        let Vec3{points: p2} = v;
        Vec3 {
            points: [
                p1[1] * p2[2] - p1[2] * p2[1],
                p1[2] * p2[0] - p1[0] * p2[2],
                p1[0] * p2[1] - p1[1] * p2[0]
            ]
        }
    }
}