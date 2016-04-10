pub mod sphere;

use image::Rgba;

pub trait HasColor {
    fn get_color(&self) -> Rgba<u8>;
}