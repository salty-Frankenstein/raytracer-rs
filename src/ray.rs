use cgmath::Vector3;
use crate::Vec3;

pub struct Ray {
    pub o: Vector3<f32>,
    pub d: Vector3<f32>,
}
impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.o + (t * self.d)
    }
}