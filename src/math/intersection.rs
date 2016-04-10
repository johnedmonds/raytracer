use math::ray::Ray;
use math::vec::Vec3;

/// An intersection between a Ray and an intersectable object.
pub struct Intersection {
    ray: Ray,
    
    /// Intersection point on the line.
    /// Specifically this is the t in the equation intersection_point = ray.direction * t + ray.origin
    pub t: f32,
}

impl Intersection {
    fn intersection_point(&self) -> Vec3 {
        let x = self.ray.origin + self.ray.direction;
        self.ray.origin + self.ray.direction * self.t
    }
}

/// An object that can be intersected by a ray.
pub trait Intersectable {
    fn intersection(ray: Ray) -> Intersection;
}