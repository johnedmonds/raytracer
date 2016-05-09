pub mod camera;
pub mod scene;

use std::vec::Vec;
use image::Rgba;
use entities::HasColor;
use entities::Entity;
use math::intersection::Intersection;
use math::ray::Ray;
use nalgebra::{distance_squared, Norm};
use raytracer::scene::Scene;

struct IntersectingEntity<'a, T: 'a + HasColor + Entity> {
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

fn find_closest_intersecting_entity<T: HasColor + Entity>(
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

pub fn trace<T:HasColor + Entity>(
    scene: &Scene<T>,
    canvas_x: i32,
    canvas_y: i32) -> Rgba<u8> {
    if scene.entities.is_empty() {
        Rgba([0, 0, 0, 0])
    } else {
        let camera_ray = Ray{
            origin: scene.camera.position,
            direction: scene.camera.from_image_coords(canvas_x, canvas_y),
        };
        let intersecting_entity = find_closest_intersecting_entity(
            camera_ray,
            scene.entities);
        match intersecting_entity {
            None => Rgba([0, 0, 0, 0]),
            Some(intersecting_entity) => {
                // Check for a shadow.
                let intersection_point = intersecting_entity.intersection.intersection_point();
                let direction_to_light = (scene.light.position - intersection_point).normalize();
                let ray_to_light = Ray{origin: intersection_point, direction: direction_to_light};
                match find_closest_intersecting_entity(ray_to_light, scene.entities) {
                    None => apply_brightness_to_color(intersecting_entity.entity.get_color(), scene.light.brightness),
                    Some(_) => Rgba{data: [0, 0, 0, 255]}
                }
            }
                
        }
    }
}