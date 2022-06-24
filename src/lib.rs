pub mod camera;
pub mod geometry;
pub mod hitable;
pub mod light;
pub mod material;
pub mod mesh;
pub mod obj_loader;
pub mod ray;
pub mod sampler;
pub mod scene;
pub mod shader;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec2 = cgmath::Vector2<f32>;
pub type Pt3 = cgmath::Point3<f32>;
pub type Pt2 = cgmath::Point2<f32>;
pub type Color = Vec3;
pub const T_MAX: f32 = 10000000.0;
pub const T_MIN: f32 = 0.001;
const ZERO: f32 = f32::EPSILON;

pub const NX: i32 = 200;
pub const NY: i32 = 200;
pub const NS: i32 = 10;
pub const NS2: i32 = 9;

pub fn mul_v(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

use cgmath::prelude::*;
pub fn vec_eq(v1: &Vec3, v2: &Vec3) -> bool {
    let d = v1 - v2;
    d.dot(d) < T_MIN
}
