use crate::hitable::*;
use crate::light::*;
use crate::ray::*;
use crate::*;
use cgmath::prelude::*;
use rand::prelude::*;

pub trait Material {
    /// create a scattered ray, or None
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;

    /// the scatter function for distributed raytracing
    fn scatter_d(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;

    /// how much the ray should be attenuated
    fn attenuation(&self) -> RGBSpectrum;
}

#[derive(Clone)]
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

    fn scatter_d(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        self.scatter(r_in, rec)
    }
    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }
}

#[derive(Clone)]
pub struct Diffuse {
    pub albedo: RGBSpectrum,
}

// see: http://www.sklogwiki.org/SklogWiki/index.php/Random_vector_on_a_sphere
fn unit_vec_on_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let (r1, r2, rsq) = loop {
        let r1 = 1.0 - 2.0 * rng.gen::<f32>();
        let r2 = 1.0 - 2.0 * rng.gen::<f32>();
        let rsq = r1.powi(2) + r2.powi(2);
        if rsq < 1.0 {
            break (r1, r2, rsq);
        }
    };

    let rh = 2.0 * (1.0 - rsq).sqrt();
    Vec3::new(r1 * rh, r2 * rh, 1.0 - 2.0 * rsq)
}

impl Material for Diffuse {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Ray> {
        // stop tracing for diffused
        None
    }

    fn scatter_d(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        // generate a random unit vector, in the semisphere of the normal vec
        let mut d = unit_vec_on_sphere();
        if d.dot(rec.normal) < 0.0 {
            d = -d;     // semisphere
        }
        Some(Ray { o: rec.p, d: d })
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

#[derive(Clone)]
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

    fn scatter_d(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        self.scatter(r_in, rec)
    }

    fn attenuation(&self) -> RGBSpectrum {
        RGBSpectrum::new(1.0, 1.0, 1.0)
    }
}
