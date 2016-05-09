extern crate raytracer;
extern crate image;
extern crate nalgebra;

use raytracer::entities::Sphere;
use image::Rgba;
use std::fs::File;
use std::path::Path;
use nalgebra::{Vector3, Point3};
use raytracer::raytracer::scene::Scene;

fn main() {
    let entities = vec![
        Sphere{
            center: Point3::new(0.0, 0.0, 3.0),
            radius: 0.5,
            color: Rgba([0, 0, 255, 255]),
        },
        Sphere{
            center: Point3::new(0.5, 1.0, 2.5),
            radius: 0.5,
            color: Rgba([0, 255, 0, 255]),
        }
    ];
    
    let camera = raytracer::raytracer::camera::Camera{
        position: Point3::new(0.0, 0.0, 0.0),
        direction: Vector3::new(0.0, 0.0, 1.0),
        image_width: 512,
        image_height: 512,
    };
    let light = raytracer::entities::Light {
        position: Point3::new(0.0, 10.0, 0.0),
        brightness: 1.0,
    };
    let mut image_buffer = image::ImageBuffer::new(camera.image_width as u32, camera.image_height as u32);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        *pixel = raytracer::raytracer::trace(
            &Scene {
                camera: &camera,
                entities: &entities,
                light: &light},
            x as i32,
            y as i32);
    }
    
    let ref mut fout = File::create(&Path::new("test.png")).unwrap();

    let _ = image::ImageRgba8(image_buffer).save(fout, image::PNG);
}