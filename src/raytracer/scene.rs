use raytracer::camera::Camera;
use entities::Entity;
use entities::HasColor;
use entities::Light;

pub struct Scene<'a, T: 'a + HasColor + Entity> {
    pub camera: &'a Camera,
    pub entities: &'a Vec<T>,
    // TODO: Only one light for now. Hopefully more later.
    pub light: &'a Light,
}