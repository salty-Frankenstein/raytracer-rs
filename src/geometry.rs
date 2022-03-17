use crate::hitable::*;
use crate::material::*;
use crate::ray::Ray;
use crate::*;
use cgmath::prelude::*;
use std::rc::Rc;

// pub struct Triangle(pub Pt3, pub Pt3, pub Pt3);
pub struct Triangle {
    pub vertex: (Pt3, Pt3, Pt3),
    // pub normal: Vec3,
    pub mat: Rc<dyn Material>,
}

// pub fn normal(a: Pt3, b: Pt3, c: Pt3) {

// }

impl Triangle {
    /// return the normal vector of the triangle
    pub fn normal(&self) -> Vec3 {
        (self.vertex.1 - self.vertex.0)
            .cross(self.vertex.2 - self.vertex.0)
            .normalize()
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let _E1 = self.vertex.1 - self.vertex.0;
        let _E2 = self.vertex.2 - self.vertex.0;
        let _P = r.d.cross(_E2);
        let det = _P.dot(_E1);
        // TODO
        if det < 0.0 {
            return None;
        }
        if det == 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;
        let _T = r.o - self.vertex.0;
        let _Q = _T.cross(_E1);
        let u = inv_det * _P.dot(_T);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let v = inv_det * _Q.dot(r.d);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inv_det * _Q.dot(_E2);
        if t - 0.0 < t_min {
            return None;
        }
        Some(HitRecord {
            t: t,
            p: r.point_at_parameter(t), // TODO
            normal: _E1.cross(_E2).normalize(),
            // normal: self.normal(),
            mat: Some(self.mat.clone())
        })
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Rc<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.o - self.center;
        let a = r.d.dot(r.d);
        let b = oc.dot(r.d);
        let c = oc.dot(oc.to_vec()) - self.radius.powi(2);
        let d = b.powi(2) - a * c;
        if d > t_min {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: t,
                    p: p,
                    normal: normal.to_vec(),
                    mat: Some(self.mat.clone())
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
                    normal: normal.to_vec(),
                    mat: Some(self.mat.clone())
                });
            }
        }
        None
    }
}
