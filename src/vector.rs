mod operations;
pub mod utility;
pub mod random;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    points: [f64; 3]
}

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

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.points[0] < s && self.points[1] < s && self.points[2] < s 
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
