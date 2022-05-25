use crate::hitable::*;
use crate::ray::*;
use crate::sampler::*;
use crate::*;
use cgmath::prelude::*;
use cgmath::Vector2;
use rand::prelude::*;
use std::f32::consts::PI;

/// the RGB spectrum, R, G, B respectively
pub type RGBSpectrum = Vec3;
pub const BLACK: RGBSpectrum = RGBSpectrum::new(0.0, 0.0, 0.0);
pub const WHITE: RGBSpectrum = RGBSpectrum::new(1.0, 1.0, 1.0);

/// trait of light sources
pub trait Light {
    /// test if the hit point is visible with the light, return the radiance if so
    fn visible(&mut self, hit_point: Pt3, normal: Vec3, world: &HitableList)
        -> Option<RGBSpectrum>;

    /// visible function, for distributed raytracing
    fn visible_d(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum>;

    /// test if a ray hits the light source
    fn hit(&self, r: &Ray) -> Option<(RGBSpectrum, f32)>;
}

/// point light source
pub struct PointLight {
    pub origin: Pt3,
    pub spectrum: RGBSpectrum,
}

impl Light for PointLight {
    fn visible(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum> {
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

    fn visible_d(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum> {
        self.visible(hit_point, normal, world)
    }

    fn hit(&self, _r: &Ray) -> Option<(RGBSpectrum, f32)> {
        None
    }
}

pub struct DiskLight {
    pub origin: Pt3,
    pub radius: f32,
    pub spectrum: RGBSpectrum,
    pub sampler_kind: SamplerKind,
    sampler: Option<Box<dyn AreaSampler>>,
}

impl DiskLight {
    fn init_sampler(&mut self) {
        if self.sampler.as_ref().map_or(false, |x| x.has_next()) {
            return;
        }
        // sample in disk of self.radius
        let sampler: Box<dyn AreaSampler> = match self.sampler_kind {
            SamplerKind::WhiteNoise => Box::new(WhiteNoiseSampler::new(self.radius * 2.0, NS2)),
            SamplerKind::Uniform => Box::new(UniformSampler::new(self.radius * 2.0, NS2)),
            SamplerKind::Jittered => Box::new(JitteredSampler::new(self.radius * 2.0, NS2)),
            SamplerKind::BlueNoise => Box::new(BlueNoiseSampler::new(self.radius * 2.0, NS2, true)),
        };
        self.sampler = Some(sampler);
    }

    pub fn new(origin: Pt3, radius: f32, spectrum: RGBSpectrum, sampler_kind: SamplerKind) -> Self {
        let mut res = DiskLight {
            origin: origin,
            radius: radius,
            spectrum: spectrum,
            sampler_kind: sampler_kind,
            sampler: None,
        };
        res.init_sampler();
        res
    }
}

impl Light for DiskLight {
    fn visible(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum> {
        // actually it's an integral, here use Monte Carlo
        // TODO: refactor
        let mut radiance = BLACK;
        self.init_sampler();
        let sampler = self.sampler.as_mut().unwrap();
        let mut actual_sample_num = 0;
        while let Some((a, b)) = sampler.sample_in_disk() {
            actual_sample_num += 1;
            let origin = self.origin + Vec3::new(a - self.radius, 0.0, b - self.radius);
            let dir = origin - hit_point;
            let r = Ray {
                o: hit_point,
                d: dir,
            };
            if world.hit(&r, T_MIN, 1.0 - T_MIN).is_none() {
                radiance += self.spectrum * dir.normalize().dot(normal.normalize()) / dir.dot(dir)
            }
        }
        // println!("<{}>", actual_sample_num);
        radiance /= actual_sample_num as f32;
        if radiance != BLACK {
            Some(radiance * PI * self.radius.powi(2))
        } else {
            None
        }
    }

    fn visible_d(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum> {
        let mut radiance = BLACK;
        // sample in disk of self.radius
        self.init_sampler();
        let sampler = self.sampler.as_mut().unwrap();
        if let Some((a, b)) = sampler.sample_in_disk() {
            let origin = self.origin + Vec3::new(a - self.radius, 0.0, b - self.radius);
            let dir = origin - hit_point;
            let r = Ray {
                o: hit_point,
                d: dir,
            };
            if world.hit(&r, T_MIN, 1.0 - T_MIN).is_none() {
                radiance = self.spectrum * dir.normalize().dot(normal.normalize()) / dir.dot(dir)
            }
        }
        if radiance != BLACK {
            Some(radiance * PI)
        } else {
            None
        }
    }

    fn hit(&self, r: &Ray) -> Option<(RGBSpectrum, f32)> {
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
            Some((self.spectrum, hit_t))
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
    fn visible(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum> {
        let mut res = BLACK;
        for l in &mut self.list {
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

    fn visible_d(
        &mut self,
        hit_point: Pt3,
        normal: Vec3,
        world: &HitableList,
    ) -> Option<RGBSpectrum> {
        // randomly pick a light in the list, and apply the corresponding `visible`
        self.list
            .choose_mut(&mut rand::thread_rng())
            .and_then(|e| e.visible_d(hit_point, normal, world))
    }

    fn hit(&self, r: &Ray) -> Option<(RGBSpectrum, f32)> {
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
