use std::vec::Vec;
use image::Rgba;
use entities::HasColor;
use entities::Light;
use math::intersection::Intersectable;
use math::intersection::Intersection;
use math::ray::Ray;
use nalgebra::{Vector3, Point3, distance_squared, Norm, Rotation3, Rotate};

/// A camera which looks at the scene.
pub struct Camera {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub image_width: i32,
    pub image_height: i32,
}

impl Camera {
    /// Returns a vector pointing at the point on the image plane defined by x and y.
    /// Returned vector will be in world space.
    /// Camera space tries to maintain a 2x2 size (-1 to 1 for width and height)
    /// but for images that aren't square, we stretch it a little bit.
    fn from_image_coords(&self, x: i32, y: i32) -> Vector3<f32> {
        let camera_space_point: Point3<f32> = Point3::new(
            (x as f32) / self.image_width as f32 * 2.0 - 1.0,
            // Image coords are up-side down from camera coords (the upper-left-hand corder for images is (0, 0) but for cameras is (-1, 1)).
            // So let's just negate the y coordinate to get everything right-side up.
            -(y as f32 / self.image_height as f32 * 2.0 - 1.0),
            1.0);
        let rotated_point = Rotation3::look_at_lh(
            &self.direction,
            &Vector3::new(0.0, 1.0, 0.0))
            .rotate(&camera_space_point);
        rotated_point.to_vector().normalize()
    }
}

struct IntersectingEntity<'a, T: 'a + HasColor + Intersectable> {
    entity: &'a T,
    intersection: Intersection,
    distance_squared: f32,
}

fn get_closest_visible_intersection(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections
        .into_iter()
        .filter(|intersection| intersection.t >= 0.0)
        .fold(None, |current_max, intersection| match current_max {
            None => {
                Some(intersection)
            },
            Some(current_max) => {
                if intersection.t < current_max.t {
                    Some(intersection)
                } else {
                    Some(current_max)
                }
            }
        })
}

fn find_closest_intersecting_entity<T: HasColor + Intersectable>(
    ray: Ray,
    entities: &Vec<T>) -> Option<IntersectingEntity<T>> {
    let mut closest_intersecting_entity: Option<IntersectingEntity<T>> = None;
    
    // We're doing comparisons so we don't need the actual distance, only
    // the squared distance. Computing sqrt() is computationally intensive
    // so let's avoid it if possible.
    for entity in entities {
        let intersections: Vec<Intersection> = entity.intersection(ray);
        let closest_visible_intersection = get_closest_visible_intersection(intersections);
        match closest_visible_intersection {
            None => {},
            Some(intersection) => {
                let distance_squared = distance_squared(&intersection.intersection_point(), &ray.origin);
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
            color.data[3],
        ]
    }
}

pub fn trace<T:HasColor + Intersectable>(
    camera: &Camera,
    canvas_x: i32,
    canvas_y: i32,
    entities: &Vec<T>,
    // TODO: Only one light for now. Hopefully more later.
    light: &Light) -> Rgba<u8> {
    if entities.is_empty() {
        Rgba([0, 0, 0, 0])
    } else {
        let camera_ray = Ray{
            origin: camera.position,
            direction: camera.from_image_coords(canvas_x, canvas_y),
        };
        let intersecting_entity = find_closest_intersecting_entity(
            camera_ray,
            entities);
        match intersecting_entity {
            None => Rgba([0, 0, 0, 0]),
            Some(intersecting_entity) => {
                // Check for a shadow.
                let intersection_point = intersecting_entity.intersection.intersection_point();
                let direction_to_light = (light.position - intersection_point).normalize();
                let ray_to_light = Ray{origin: intersection_point, direction: direction_to_light};
                match find_closest_intersecting_entity(ray_to_light, entities) {
                    None => apply_brightness_to_color(intersecting_entity.entity.get_color(), light.brightness),
                    Some(_) => Rgba{data: [0, 0, 0, 255]}
                }
            }
                
        }
    }
}