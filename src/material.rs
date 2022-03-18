use crate::hitable::*;
use crate::light::*;
use crate::ray::*;
use crate::*;
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
            d: reflected,
        };
        // TODO: figure out the formula
        if scattered.d.dot(rec.normal) > f32::EPSILON {
            Some(scattered)
        } else {
            None
        }
    }

    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }
}

pub struct Diffuse {
    pub albedo: RGBSpectrum,
}

impl Material for Diffuse {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Ray> {
        // stop tracing for diffused
        None
    }

    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > T_MIN {
        Some(ni_over_nt * (uv - n * dt) - n * d.sqrt())
    } else {
        None
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        let reflected = reflect(r_in.d, rec.normal);
        let outward_normal;
        let ni_over_nt;
        if r_in.d.dot(rec.normal) > T_MIN {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
        }
        match refract(r_in.d, outward_normal, ni_over_nt) {
            Some(refracted) => Some(Ray {
                o: rec.p,
                d: refracted,
            }),
            None => Some(Ray {
                o: rec.p,
                d: reflected,
            }),
        }
    }

    fn attenuation(&self) -> RGBSpectrum {
        RGBSpectrum::new(1.0, 1.0, 1.0)
    }
}
