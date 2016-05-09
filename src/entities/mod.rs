use image::Rgba;
use math::ray::Ray;
use math::intersection::Intersection;
use nalgebra::{Vector3, Point3, Dot};

pub trait HasColor {
    fn get_color(&self) -> Rgba<u8>;
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub color: Rgba<u8>,
}

impl Entity for Sphere {
    fn intersection(&self, ray: Ray) -> Vec<Intersection> {
        // So we want to find the intersection between ray and self (a sphere).
        // We know that the equation for a sphere is x^2 + y^2 + z^2 = r^2
        // where x, y, and z are the coordinates of each point on the sphere
        // and r is the radius of the sphere.
        //
        // We can actually simplify this and say P^2 = r^2 where P is a vector [x, y, z]
        // and represents all points P on the sphere.
        //
        // Now that's the equation for a sphere centered at the origin. We want a
        // sphere centered anywhere (for example at the point C). But to make everything
        // work, we just move this sphere back to the origin by subtracting C from
        // all points P
        //
        // The new equation becomes (P - C)^2 = r^2.
        //
        // Now we want to intersect the given ray with the sphere. The equation for
        // a ray is O + tD where O is the ray's origin, D is the ray's direction,
        // and t is the distance along the ray from O in the direction D.
        // We want to find t for which the ray intersects the circle (or find that
        // no such intersection exists). We can just set P = ray = O + tD.
        // This plugs right back into the original equation which becomes
        // (O + tD - C)^2 - r^2 = 0
        // We can rewrite this as (tD + (O - C))^2 - r^2 = 0
        // After a bunch of multiplying, we end up with
        // t^2D^2 + 2tD(O - C) + (O - C)^2 - r^2 = 0.
        //
        // Now if you squint, this looks a bit like a quadratic equation involving t.
        // That's good for us since solving for t otherwise is pretty hard.
        // Recall the a, b, and c of quadratic equation fame. In this context they
        // become a = 1, b = 2D(O - C), and c = (O - C)^2 - r^2
        // Plugging it into the quadratic equation gives us our solutions (or
        // tells us no solutions exist).
        
        let a: f32 = 1.0;
        let center_to_ray: Vector3<f32> = ray.origin - self.center;
        let b: f32 = 2.0 * ray.direction.dot(&center_to_ray);
        let c: f32 = center_to_ray.dot(&center_to_ray) - self.radius * self.radius;

        let discriminate: f32 = b * b - 4.0 * a * c;
        if discriminate < 0.0 {
            vec!()
        } else {
            let discriminate_sqrt = discriminate.sqrt();
            let negative_intersection = Intersection{ray: ray, t: (- b - discriminate_sqrt) / 2.0};
            if discriminate == 0.0 {
                vec!(negative_intersection)
            } else {
                let positive_intersection = Intersection{ray: ray, t: (-b + discriminate_sqrt) / 2.0};
                vec!(negative_intersection, positive_intersection)
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
    pub position: Point3<f32>,
    // TODO: Should be color but let's just use brighness for now.
    pub brightness: f32,
}

/// A physical entity in the scene (e.g. a ball or train, etc. but not a light or camera).
pub trait Entity: HasColor {
    fn intersection(&self, ray: Ray) -> Vec<Intersection>;
}