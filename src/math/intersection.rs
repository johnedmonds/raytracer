use math::ray::Ray;
use nalgebra::*;

/// An intersection between a Ray and an intersectable object.
pub struct Intersection {
    pub ray: Ray,
    
    /// Intersection point on the line.
    /// Specifically this is the t in the equation intersection_point = ray.direction * t + ray.origin
    pub t: f32,
}

impl Intersection {
    pub fn intersection_point(&self) -> Point3<f32> {
        (self.ray.origin.to_vector() + (self.ray.direction * self.t)).to_point()
    }
}

/// An object that can be intersected by a ray.
pub trait Intersectable {
    fn intersection(&self, ray: Ray) -> Vec<Intersection>;
}