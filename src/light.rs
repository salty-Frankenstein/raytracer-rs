use crate::hitable::*;
use crate::ray::*;
use crate::*;
use cgmath::prelude::*;

/// the RGB spectrum, R, G, B respectively
pub type RGBSpectrum = Vec3;
pub const BLACK: RGBSpectrum = RGBSpectrum::new(0.0, 0.0, 0.0);

/// trait of light sources
pub trait Light {
    /// test if the hit point is visible with the light, return the radiance if so
    fn visible(&self, hit_point: Pt3, normal: Vec3, world: &HitableList) -> Option<RGBSpectrum>;
}

/// point light source
pub struct PointLight {
    pub origin: Pt3,
    pub spectrum: RGBSpectrum,
}

impl Light for PointLight {
    fn visible(&self, hit_point: Pt3, normal: Vec3, world: &HitableList) -> Option<RGBSpectrum> {
        // a ray from the hitting point to the light origin
        let dir = self.origin - hit_point;
        let r = Ray {
            o: hit_point,
            d: dir,
        };
        // if hit something, then it is invisible
        match world.hit(&r, T_MIN, T_MAX) {
            Some(_) => None,
            None => Some(self.spectrum 
                // unit vector dir dot normal is the cosine of the angle
                * dir.normalize().dot(normal.normalize())   // lambert's law
                // dir dot dir is the squared length to the light
                / dir.dot(dir)                              // inverse square law
            ),
        }
    }
}

/// list of lights
pub struct LightList {
    pub list: Vec<Box<dyn Light>>,
}

impl Light for LightList {
    fn visible(&self, hit_point: Pt3, normal: Vec3, world: &HitableList) -> Option<RGBSpectrum> {
        let mut res = BLACK;
        for l in &self.list {
            if let Some(r) = l.visible(hit_point, normal, world) {
                res += r;
            }
        }
        if res != BLACK {
            Some(res)
        } else {
            None
        }
    }
}
