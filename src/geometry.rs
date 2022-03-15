use crate::hitable::*;
use crate::ray::Ray;
use crate::Vec3;
use cgmath::prelude::*;
pub type Vertex = Vec3;

pub struct Triangle(Vertex, Vertex, Vertex);

impl Triangle {
    /// return the normal vector of the triangle
    pub fn normal(&self) -> Vec3 {
        (self.1 - self.0).cross(self.2 - self.0).normalize()
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.o - self.center;
        let a = r.d.dot(r.d);
        let b = oc.dot(r.d);
        let c = oc.dot(oc) - self.radius.powi(2);
        let d = b.powi(2) - a * c; 
        if d > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: t,
                    p: p,
                    normal: normal,
                });
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: t,
                    p: p,
                    normal: normal,
                });
            }
        }
        None
    }
}
