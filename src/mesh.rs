use crate::geometry::Triangle;
use crate::hitable::*;
use crate::ray::*;
use crate::Vec3;
use crate::*;
use cgmath::prelude::*;
use cgmath::*;

pub trait FromFaceList {
    fn from_face_list(list: &Vec<Triangle>) -> Self;
}

/// Mesh is a struct with a Hitable list and an accelerate structure
/// for multiple implementations
pub struct Mesh<T: FromFaceList> {
    pub face_list: Vec<Triangle>,
    pub acc_structure: T,
}

impl<T: FromFaceList> Mesh<T> {
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
            f.vertex.0 = d.transform_point(f.vertex.0);
            f.vertex.1 = d.transform_point(f.vertex.1);
            f.vertex.2 = d.transform_point(f.vertex.2);
        }

        self.acc_structure = FromFaceList::from_face_list(&self.face_list)
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

/// naive implementation
pub struct Naive;
pub type NaiveMesh = Mesh<Naive>;

impl Hitable for NaiveMesh {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        hit_list(&self.face_list, r, t_min, t_max)
    }
}

impl FromFaceList for Naive {
    fn from_face_list(_list: &Vec<Triangle>) -> Self {
        Naive
    }
}

/// Optimized Mesh structure
pub type FastMesh = Mesh<Tree>;
pub type BoxMesh = Mesh<BoundingBox>;

/// Axis-aligned bounding box
pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    pub fn hit_box(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;
        for a in 0..3 {
            let t0 = ((self.min[a] - r.o[a]) / r.d[a]).min((self.max[a] - r.o[a]) / r.d[a]);
            let t1 = ((self.min[a] - r.o[a]) / r.d[a]).max((self.max[a] - r.o[a]) / r.d[a]);
            tmin = t0.max(tmin);
            tmax = t1.min(tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

impl FromFaceList for BoundingBox {
    fn from_face_list(list: &Vec<Triangle>) -> Self {
        let mut min = Vec3::new(T_MAX, T_MAX, T_MAX);
        let mut max = -min;
        for t in list {
            for a in 0..3 {
                max[a] = max[a].max(t.vertex.0[a]);
                min[a] = min[a].min(t.vertex.0[a]);
                max[a] = max[a].max(t.vertex.1[a]);
                min[a] = min[a].min(t.vertex.1[a]);
                max[a] = max[a].max(t.vertex.2[a]);
                min[a] = min[a].min(t.vertex.2[a]);
            }
        }
        println!(
            "Box: max({}, {}, {}), min({}, {}, {})",
            max[0], max[1], max[2], min[0], min[1], min[2]
        );
        BoundingBox { max: max, min: min }
    }
}

impl Hitable for Mesh<BoundingBox> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.acc_structure.hit_box(r, t_min, t_max) {
            hit_list(&self.face_list, r, t_min, t_max)
        } else {
            None
        }
    }
}

pub struct Tree {}
