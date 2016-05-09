use nalgebra::{Vector3, Point3, Norm, Rotation3, Rotate};

/// A camera which looks at the scene.
pub struct Camera {
    pub position: Point3<f32>,
    pub rotation: Rotation3<f32>,
    pub image_width: i32,
    pub image_height: i32,
}

impl Camera {
    /// Returns a vector pointing at the point on the image plane defined by x and y.
    /// Returned vector will be in world space.
    /// Camera space tries to maintain a 2x2 size (-1 to 1 for width and height)
    /// but for images that aren't square, we stretch it a little bit.
    pub fn from_image_coords(&self, x: i32, y: i32) -> Vector3<f32> {
        let camera_space_point: Point3<f32> = Point3::new(
            (x as f32) / self.image_width as f32 * 2.0 - 1.0,
            // Image coords are up-side down from camera coords (the upper-left-hand corder for images is (0, 0) but for cameras is (-1, 1)).
            // So let's just negate the y coordinate to get everything right-side up.
            -(y as f32 / self.image_height as f32 * 2.0 - 1.0),
            1.0);
        let rotated_point = self.rotation.rotate(&camera_space_point);
        rotated_point.to_vector().normalize()
    }
}