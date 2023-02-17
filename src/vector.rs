pub mod vector {
    use std::ops;

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

        pub fn length(&self) -> f64 {
            let Vec3 { points } = self;
            let t = points[0] * points[0] + points[1] * points[1] + points[2] * points[2];
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