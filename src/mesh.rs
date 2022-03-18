use crate::geometry::Triangle;
use crate::hitable::*;
use crate::ray::*;
use crate::Vec3;
use cgmath::prelude::*;
use cgmath::*;

/// a funtor-like trait, for transformation
pub trait MapTriangle {
    fn map_t<T: Fn(&Triangle) -> Triangle>(&mut self, f: T);
}

/// Mesh is a struct with a Hitabble & Mappable face_list,
/// for multiple implementations
pub struct Mesh<T: Hitable + MapTriangle> {
    // TODO: Accelerations
    pub face_list: T,
}

impl<T: Hitable + MapTriangle> Mesh<T> {
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
        self.face_list.map_t(|f| Triangle {
            vertex: (
                d.transform_point(f.vertex.0),
                d.transform_point(f.vertex.1),
                d.transform_point(f.vertex.2),
            ),
            mat: f.mat.clone(),
        })
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

impl<T:Hitable+MapTriangle> Hitable for Mesh<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.face_list.hit(r, t_min, t_max)
    }
}

/// instantiation for Vec<Triangle>
pub type NaiveMesh = Mesh<Vec<Triangle>>;

/// implement Hitable & MapTriangle for Vec<Triangle>
impl Hitable for Vec<Triangle> {
    // TODO: Accelerations
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = EMPTY_REC;
        for i in self {
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

impl MapTriangle for Vec<Triangle> {
    fn map_t<T: Fn(&Triangle) -> Triangle>(&mut self, f: T) {
        for t in self {
            *t = f(t);
        }
    }
}

// /// Optimized Mesh structure
// pub struct FastMesh {
//     mesh:
// }
