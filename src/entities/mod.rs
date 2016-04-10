use image::Rgba;
use math::vec::Vec3;
use math::ray::Ray;
use math::intersection::{Intersection, Intersectable};

pub trait HasColor {
    fn get_color(&self) -> Rgba<u8>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Rgba<u8>,
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let center_to_origin = self.center - ray.origin;
        // tca is the distance along ray where we intersect with a perpendictular
        // line going through the sphere center.
        let tca = center_to_origin.dot(ray.direction);
        
        // Check if the intersection is behind the camera.
        if tca < 0.0 {
            None
        } else {
            // d is the distance from tca (or at least the point at distance tca from the origin along ray) to the sphere center.
            let d_squared = center_to_origin.len() - tca * tca;
            
            // Check if the "intersection" happens outside the sphere.
            if d_squared > self.radius * self.radius {
                None
            } else {
                // This gives us the distance from tca to the intersection points (yes points, plural).
                // However, we always take the negative one since that's the one closest to the camera.
                let thc = -(self.radius * self.radius - d_squared).sqrt();

                // Remember that thc is guaranteed to be negative here so this is really more like tca - thc.
                let distance_to_nearest_intersection = tca + thc;
                
                Some(Intersection{ray: ray.clone(), t: distance_to_nearest_intersection})
            }
        }
    }
}

impl HasColor for Sphere {
    fn get_color(&self) -> Rgba<u8> {
        self.color
    }
}

pub struct Light {
    pub position: Vec3,
    // TODO: Should be color but let's just use brighness for now.
    pub brightness: f32,
}
