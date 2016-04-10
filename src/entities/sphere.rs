use math::vec::Vec3;
use math::ray::Ray;
use math::intersection::{Intersection, Intersectable};

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: Ray) -> Intersection {
        let center_to_origin = self.center - ray.origin;
        // tca is the distance along ray where we intersect with a perpendictular
        // line going through the sphere center.
        let tca = center_to_origin.dot(ray.direction);
        
        // d is the distance from tca (or at least the point at distance tca from the origin along ray) to the sphere center.
        let d_squared = center_to_origin.len() - tca * tca;
        
        // This gives us the distance from tca to the intersection points (yes points, plural).
        // However, we always take the negative one since that's the one closest to the camera.
        let thc = -(self.radius * self.radius - d_squared).sqrt();

        // Remember that thc is guaranteed to be negative here so this is really more like tca - thc.
        let distance_to_nearest_intersection = tca + thc;
        
        Intersection{ray: ray.clone(), t: distance_to_nearest_intersection}
    }
}