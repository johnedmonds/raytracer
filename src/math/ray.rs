use math::vec::Vec3;

// A ray in a scene
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}