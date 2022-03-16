use crate::light::*;
use crate::material::*;
use crate::ray::Ray;
use crate::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Pt3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
}

pub const EMPTY_REC: HitRecord = HitRecord {
    t: 0.0,
    p: Pt3::new(0.0, 0.0, 0.0),
    normal: Vec3::new(0.0, 0.0, 0.0),
    mat: None
};

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = EMPTY_REC;
        for i in &self.list {
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
