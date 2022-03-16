use crate::geometry::Triangle;
use crate::hitable::*;
use crate::ray::*;
use crate::Vec3;
use cgmath::prelude::*;
use cgmath::*;

pub struct Mesh {
    // TODO: Accelerations
    pub face_list: Vec<Triangle>,
}

impl Hitable for Mesh {
    // TODO: Accelerations
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = EMPTY_REC;
        for i in &self.face_list {
            if let Some(temp_rec) = i.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}

impl Mesh {
    pub fn transform(&mut self, scale: f32, disp: Vec3, x: f32, y: f32, z: f32) {
        let rotation = Quaternion::from(Euler {
            x: Deg(x),
            y: Deg(y),
            z: Deg(z),
        });
        let d = Decomposed {
            scale: scale,
            rot: rotation,
            disp: disp,
        };
        for f in &mut self.face_list {
            f.0 = d.transform_point(f.0);
            f.1 = d.transform_point(f.1);
            f.2 = d.transform_point(f.2);
        }
    }

    pub fn scale(&mut self, scale: f32) {
        self.transform(scale, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0);
    }

    /// perform a displacement with a given vec3
    pub fn displacement(&mut self, disp: Vec3) {
        self.transform(1.0, disp, 0.0, 0.0, 0.0);
    }

    /// preform a rotation with a given euler angle
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.transform(1.0, Vec3::new(0.0, 0.0, 0.0), x, y, z);
    }
}
