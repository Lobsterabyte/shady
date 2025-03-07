use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Vec3 { x, y, z };
    }

    pub fn length(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    return Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        return self * (1.0 / rhs);
    }
}
