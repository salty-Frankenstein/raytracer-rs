use crate::*;

pub struct Ray {
    pub o: Pt3,
    pub d: Vec3,
}
impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Pt3 {
        self.o + (t * self.d)
    }
}