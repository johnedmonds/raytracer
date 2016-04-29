use nalgebra::Vector3;
use nalgebra::Point3;

// A ray in a scene
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}