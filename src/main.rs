extern crate raytracer;
extern crate image;

use raytracer::entities::Sphere;
use raytracer::math::vec::Vec3;
use image::Rgba;
use std::fs::File;
use std::path::Path;

fn main() {
    let entities = vec![
        Sphere{
            center: Vec3{
                x: 0.0,
                y: 0.0,
                z: 4.0,
            },
            radius: 1.0,
            color: Rgba([0, 255, 0, 255]),
        }
    ];
    
    let camera = raytracer::raytracer::Camera{
        position: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        direction: Vec3 {x: 0.0, y: 0.0, z: 1.0},
        image_width: 512,
        image_height: 512,
    };
    let light = raytracer::entities::Light {
        position: Vec3 {x: 0.0, y: 10.0, z: 0.0},
        brightness: 1.0,
    };
    let mut image_buffer = image::ImageBuffer::new(camera.image_width as u32, camera.image_height as u32);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        *pixel = raytracer::raytracer::trace(&camera, x as i32, y as i32, &entities, &light);
    }
    
    let ref mut fout = File::create(&Path::new("test.png")).unwrap();

    let _ = image::ImageRgba8(image_buffer).save(fout, image::PNG);
}