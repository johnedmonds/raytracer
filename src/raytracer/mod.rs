use std::vec::Vec;
use image::Rgba;
use entities::HasColor;
use entities::Light;
use math::intersection::Intersectable;
use math::intersection::Intersection;
use math::vec::Vec3;
use math::ray::Ray;

/// A camera which looks at the scene.
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub image_width: i32,
    pub image_height: i32,
}

impl Camera {
    /// Move a point from image space into camera space.
    /// Camera space tries to maintain a 2x2 size (-1 to 1 for width and height)
    /// but for images that aren't square, we stretch it a little bit.
    fn from_image_coords(&self, x: i32, y: i32) -> Vec3 {
        Vec3 {
            x: x as f32 / self.image_width as f32 * 2.0 - 1.0,
            y: y as f32 / self.image_height as f32 * 2.0 - 1.0,
            z: 0.0}
            + self.position
    }
}

struct IntersectingEntity<'a, T: 'a + HasColor + Intersectable> {
    entity: &'a T,
    intersection: Intersection,
    distance_squared: f32,
}

fn find_closest_intersecting_entity<T: HasColor + Intersectable>(
    ray: Ray,
    entities: &Vec<T>) -> Option<IntersectingEntity<T>> {
    let mut closest_intersecting_entity: Option<IntersectingEntity<T>> = None;
    
    // We're doing comparisons so we don't need the actual distance, only
    // the squared distance. Computing sqrt() is computationally intensive
    // so let's avoid it if possible.
    for entity in entities {
        let intersection = entity.intersection(ray);
        match intersection {
            None => {},
            Some(intersection) => {
                let distance_squared = (intersection.intersection_point() - ray.origin).len_squared();
                closest_intersecting_entity = match closest_intersecting_entity {
                    None => Some(IntersectingEntity{
                        entity: entity,
                        intersection: intersection,
                        distance_squared: distance_squared,
                    }),
                    Some(closest_intersecting_entity) =>
                        if distance_squared < closest_intersecting_entity.distance_squared {
                            Some(IntersectingEntity{
                            entity: entity,
                            intersection: intersection,
                            distance_squared: distance_squared})
                        } else {
                            Some(closest_intersecting_entity)
                        }
                }
            }
        }
    }
    
    closest_intersecting_entity
}

fn apply_brightness_to_color(color: Rgba<u8>, brightness: f32) -> Rgba<u8> {
    Rgba{
        data: [
            (color.data[0] as f32 * brightness) as u8,
            (color.data[1] as f32 * brightness) as u8,
            (color.data[2] as f32 * brightness) as u8,
            (color.data[3] as f32 * brightness) as u8,            
        ]
    }
}

pub fn trace<T:HasColor + Intersectable>(
    camera: Camera,
    canvas_x: i32,
    canvas_y: i32,
    entities: &Vec<T>,
    // TODO: Only one light for now. Hopefully more later.
    light: Light) -> Rgba<u8> {
    if entities.is_empty() {
        Rgba([0, 0, 0, 0])
    } else {
        let camera_ray = Ray{
            origin: camera.from_image_coords(canvas_x, canvas_y),
            direction: camera.direction};
        let intersecting_entity = find_closest_intersecting_entity(
            camera_ray,
            entities);
        match intersecting_entity {
            None => Rgba([0, 0, 0, 0]),
            Some(intersecting_entity) => {
                // Check for a shadow.
                let intersection_point = intersecting_entity.intersection.intersection_point();
                let direction_to_light = (light.position - intersection_point).normalized();
                let ray_to_light = Ray{origin: intersection_point, direction: direction_to_light};
                match find_closest_intersecting_entity(ray_to_light, entities) {
                    None => apply_brightness_to_color(intersecting_entity.entity.get_color(), light.brightness),
                    Some(_) => Rgba{data: [0, 0, 0, 0]}
                }
            }
                
        }
    }
}