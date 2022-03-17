use crate::ray::*;
use crate::*;
use cgmath::prelude::*;

pub struct Camera {
    origin: Pt3,
    lower_left_corner: Pt3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Pt3, lookat: Pt3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: origin,
            lower_left_corner: Pt3::new(-half_width, -half_height, -1.0),
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            o: self.origin,
            d: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        }
    }
}
