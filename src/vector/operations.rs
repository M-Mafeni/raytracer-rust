use std::ops;
use super::Vec3;

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
