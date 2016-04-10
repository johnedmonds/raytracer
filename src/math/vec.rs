use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Vec3) -> Vec3{
        Vec3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x}
    }
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalized(&self) -> Vec3 {
        let len: f32 = self.len();
        Vec3{x: self.x / len, y: self.y / len, z: self.z / len}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}