use crate::hitable::*;
use crate::ray::*;
use crate::*;
use cgmath::prelude::*;
use cgmath::Vector2;
use rand::prelude::*;
use std::f32::consts::PI;

/// the RGB spectrum, R, G, B respectively
pub type RGBSpectrum = Vec3;
pub const BLACK: RGBSpectrum = RGBSpectrum::new(0.0, 0.0, 0.0);

/// trait of light sources
pub trait Light {
    /// test if the hit point is visible with the light, return the radiance if so
    fn visible(&self, hit_point: Pt3, normal: Vec3, world: &HitableList) -> Option<RGBSpectrum>;

    /// test if a ray hits the light source
    fn hit(&self, r: &Ray) -> Option<RGBSpectrum>;
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
        // if hit something , then it is invisible
        // tmax > 1.0 means that the hit point is behind the light source
        match world.hit(&r, T_MIN, 1.0 - T_MIN) {
            Some(_) => None,
            // lambert's law & inverse square law
            // unit vector dir dot normal is the cosine of the angle
            // dir dot dir is the squared length to the light
            None => Some(self.spectrum * dir.normalize().dot(normal.normalize()) / dir.dot(dir)),
        }
    }

    fn hit(&self, _r: &Ray) -> Option<RGBSpectrum> {
        None
    }
}

pub struct DiskLight {
    pub origin: Pt3,
    pub radius: f32,
    pub spectrum: RGBSpectrum,
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), 0.0, rng.gen()) - Vec3::new(1.0, 0.0, 1.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Light for DiskLight {
    fn visible(&self, hit_point: Pt3, normal: Vec3, world: &HitableList) -> Option<RGBSpectrum> {
        // actually it's an integral, here use Monte Carlo
        // TODO: refactor
        let mut radiance = BLACK;
        for _ in 0..NS2 {
            let origin = self.origin + random_in_unit_disk() * self.radius;
            let dir = origin - hit_point;
            let r = Ray {
                o: hit_point,
                d: dir,
            };
            if world.hit(&r, T_MIN, 1.0 - T_MIN).is_none() {
                radiance += self.spectrum * dir.normalize().dot(normal.normalize()) / dir.dot(dir)
            }
        }
        radiance /= NS2 as f32;
        if radiance != BLACK {
            Some(radiance * PI * self.radius.powi(2))
        } else {
            None
        }
    }

    fn hit(&self, r: &Ray) -> Option<RGBSpectrum> {
        let hit_t = (self.origin.y - r.o.y) / r.d.y;
        if hit_t < T_MIN {
            return None;
        }
        let hit_x = r.o.x + hit_t * r.d.x;
        let hit_z = r.o.z + hit_t * r.d.z;
        let vh = Vector2::new(hit_x, hit_z);
        let vo = Vector2::new(self.origin.x, self.origin.z);
        let rt = vh - vo;
        if rt.dot(rt) <= self.radius.powi(2) {
            Some(self.spectrum)
        } else {
            None
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
    fn hit(&self, r: &Ray) -> Option<RGBSpectrum> {
        // TODO: refactor
        for l in &self.list {
            let t = l.hit(r);
            if t.is_some() {
                return t;
            }
        }
        return None;
    }
}
