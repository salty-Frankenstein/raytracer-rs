use crate::geometry::Triangle;
use crate::hitable::*;
use crate::ray::*;
use crate::Vec3;
use crate::*;
use cgmath::prelude::*;
use cgmath::*;
use rand::prelude::*;
use std::cmp::*;

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

        // recompute the acc_structure
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

// TODO: refactor & add this method to all Mesh types
impl NaiveMesh {
    pub fn hit_both_side(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = EMPTY_REC;
        for i in self.face_list.iter() {
            if let Some(temp_rec) = i.hit_both_side(r, t_min, closest_so_far) {
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
pub type FastMesh = Mesh<BVHTree>;
pub type BoxMesh = Mesh<BoundingBox>;

/// Axis-aligned bounding box
#[derive(Clone, Copy)]
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

    fn triangle_bounding_box(t: &Triangle) -> BoundingBox {
        let mut min = Vec3::new(T_MAX, T_MAX, T_MAX);
        let mut max = -min;
        for a in 0..3 {
            max[a] = max[a].max(t.vertex.0[a]);
            min[a] = min[a].min(t.vertex.0[a]);
            max[a] = max[a].max(t.vertex.1[a]);
            min[a] = min[a].min(t.vertex.1[a]);
            max[a] = max[a].max(t.vertex.2[a]);
            min[a] = min[a].min(t.vertex.2[a]);
        }
        BoundingBox { max: max, min: min }
    }

    fn surrounding_box(b0: BoundingBox, b1: BoundingBox) -> BoundingBox {
        let small = Vec3::new(
            b0.min.x.min(b1.min.x),
            b0.min.y.min(b1.min.y),
            b0.min.z.min(b1.min.z),
        );
        let big = Vec3::new(
            b0.max.x.max(b1.max.x),
            b0.max.y.max(b1.max.y),
            b0.max.z.max(b1.max.z),
        );
        BoundingBox {
            min: small,
            max: big,
        }
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

pub enum BVHTree {
    Node {
        left: Box<BVHTree>,
        right: Box<BVHTree>,
        aabb: BoundingBox,
    },
    Leaf((BoundingBox, usize)), // the index of the original facelist
}

fn rand3() -> i32 {
    let mut rng = rand::thread_rng();
    let f = rng.gen::<f32>() * 3.0;
    if f < 1.0 {
        0
    } else if f < 2.0 {
        1
    } else {
        2
    }
}

fn box_compare(a: BoundingBox, b: BoundingBox, axis: usize) -> Ordering {
    a.min[axis].partial_cmp(&b.min[axis]).unwrap()
}

impl BVHTree {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            BVHTree::Node {
                left: _,
                right: _,
                aabb,
            } => *aabb,
            BVHTree::Leaf((aabb, _)) => *aabb,
        }
    }

    fn build_tree(face_list: &Vec<(BoundingBox, usize)>) -> Self {
        let left;
        let right;
        let axis = rand3() as usize;
        match face_list.len() {
            // only one element, then copy for both
            1 => {
                left = BVHTree::Leaf(face_list[0]);
                right = BVHTree::Leaf(face_list[0]);
            }
            2 => match box_compare(face_list[0].0, face_list[1].0, axis) {
                Ordering::Less => {
                    left = BVHTree::Leaf(face_list[0]);
                    right = BVHTree::Leaf(face_list[1]);
                }
                _ => {
                    left = BVHTree::Leaf(face_list[1]);
                    right = BVHTree::Leaf(face_list[0]);
                }
            },
            _ => {
                let mut new_list = face_list.clone();
                new_list.sort_by(|a, b| box_compare(a.0, b.0, axis));
                let mid = new_list.len() / 2;
                let (l, r) = new_list.split_at(mid);
                left = BVHTree::build_tree(&l.to_vec());
                right = BVHTree::build_tree(&r.to_vec());
            }
        }

        let aabb = BoundingBox::surrounding_box(left.bounding_box(), right.bounding_box());
        BVHTree::Node {
            left: Box::new(left),
            right: Box::new(right),
            aabb: aabb,
        }
    }

    fn hit_tree(
        &self,
        face_list: &Vec<Triangle>,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        match &self {
            BVHTree::Leaf((_, i)) => face_list[*i].hit(r, t_min, t_max),
            BVHTree::Node { left, right, aabb } => {
                if aabb.hit_box(r, t_min, t_max) {
                    let hit_left = left.hit_tree(face_list, r, t_min, t_max);
                    let hit_right = right.hit_tree(face_list, r, t_min, t_max);
                    match (hit_left, hit_right) {
                        (Some(left_rec), Some(right_rec)) => {
                            if left_rec.t < right_rec.t {
                                Some(left_rec)
                            } else {
                                Some(right_rec)
                            }
                        }
                        (Some(left_rec), None) => Some(left_rec),
                        (None, Some(right_rec)) => Some(right_rec),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl FromFaceList for BVHTree {
    fn from_face_list(list: &Vec<Triangle>) -> Self {
        let box_list = list.iter().map(BoundingBox::triangle_bounding_box);
        let mlist: Vec<_> = box_list.zip(0..).map(|(a, b)| (a, b as usize)).collect();
        BVHTree::build_tree(&mlist)
    }
}

impl Hitable for Mesh<BVHTree> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.acc_structure
            .hit_tree(&self.face_list, r, t_min, t_max)
    }
}
