use crate::hitable::*;
use crate::light::*;
use crate::*;
use crate::ray::*;
use cgmath::prelude::*;

pub trait Material {
    /// create a scattered ray, or None
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;
    /// how much the ray should be attenuated
    fn attenuation(&self) -> RGBSpectrum;
}

pub struct Metal {
    pub albedo: RGBSpectrum,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        let reflected = reflect(r_in.d.normalize(), rec.normal);
        let scattered = Ray {
            o: rec.p,
            d: reflected
        };
        // TODO: figure out the formula
        if scattered.d.dot(rec.normal) > f32::EPSILON {
            Some(scattered)
        }
        else {
            None
        }
    }

    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }
}

pub struct Diffuse {
    pub albedo: RGBSpectrum
}

impl Material for Diffuse {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        // stop tracing for diffused
        None
    }

    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }
}