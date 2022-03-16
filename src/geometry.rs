use crate::hitable::*;
use crate::ray::Ray;
use crate::*;
use cgmath::prelude::*;

pub struct Triangle(pub Pt3, pub Pt3, pub Pt3);

impl Triangle {
    /// return the normal vector of the triangle
    pub fn normal(&self) -> Vec3 {
        (self.1 - self.0).cross(self.2 - self.0).normalize()
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let _E1 = self.1 - self.0;
        let _E2 = self.2 - self.0;
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
        let _T = r.o - self.0;
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
        if t - 0.0 < f32::EPSILON  {
            return None;
        }
        Some(HitRecord {
            t: t,
            p: r.point_at_parameter(t), // TODO
            normal: _E1.cross(_E2).normalize(),
            // normal: self.normal(),
        })
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
        let c = oc.dot(oc.to_vec()) - self.radius.powi(2);
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
                    normal: normal.to_vec(),
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
                });
            }
        }
        None
    }
}
