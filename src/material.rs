use crate::hitable::*;
use crate::light::*;
use crate::ray::*;
use crate::shader::*;
use crate::*;
use cgmath::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub trait Material {
    /// create a scattered ray, or None
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;

    /// the scatter function for distributed raytracing
    /// it should follow the PDF of the material's BRDF
    fn scatter_d(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;

    /// it is not actually the `real` brdf, but the ratio of input and output power
    /// it returns an attenuation of RGBSpectrum
    fn brdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> RGBSpectrum;

    /// the pdf of the material, according to the brdf
    fn pdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> f32;

    /// how much the ray should be attenuated
    fn attenuation(&self) -> RGBSpectrum;

    fn do_material(&self, r: &Ray, rec: &HitRecord, world: &mut World, depth: i32) -> RGBSpectrum;
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: RGBSpectrum,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn assert_unit(din: Vec3) {
    debug_assert!(
        (din.dot(din) - 1.0).abs() < T_MIN,
        "din: {} {} {}, |din| {}",
        din.x,
        din.y,
        din.z,
        din.dot(din)
    );
}

impl Metal {
    fn _brdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> Option<RGBSpectrum> {
        // for metal, only the input pairs following the reflection law count
        let din = din.normalize();
        let dout = dout.normalize();
        // let dnor = dnor.normalize();

        assert_unit(din);
        assert_unit(dout);
        assert_unit(dnor);
        // TODO: check this formula
        if (din + dout).dot(dnor).abs() < T_MIN {
            Some(self.albedo)
        } else {
            None
        }
    }
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

    fn brdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> RGBSpectrum {
        if let Some(x) = self._brdf(din, dout, dnor) {
            x
        } else {
            BLACK
        }
    }

    fn pdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> f32 {
        if let Some(_) = self._brdf(din, dout, dnor) {
            1.0
        } else {
            ZERO
        }
    }

    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }

    // HACK: This method is only for the MIS shader, they are mutual recursive
    // because the calculation for the microfacet model is slightly different from others
    // it caculates the corresponding light value according to the material's BRDF
    fn do_material(&self, r: &Ray, rec: &HitRecord, world: &mut World, depth: i32) -> RGBSpectrum {
        do_material_default(self, r, rec, world, depth)
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
            d = -d; // semisphere
        }
        Some(Ray { o: rec.p, d: d })
    }

    fn brdf(&self, _din: Vec3, _dout: Vec3, _dnor: Vec3) -> RGBSpectrum {
        self.albedo / PI
    }

    fn pdf(&self, _din: Vec3, _dout: Vec3, _dnor: Vec3) -> f32 {
        // TODO: check this
        1.0 / PI
    }

    fn attenuation(&self) -> RGBSpectrum {
        self.albedo
    }

    fn do_material(&self, r: &Ray, rec: &HitRecord, world: &mut World, depth: i32) -> RGBSpectrum {
        do_material_default(self, r, rec, world, depth)
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

impl Dielectric {
    fn _brdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> Option<RGBSpectrum> {
        let outward_normal;
        let ni_over_nt;
        if din.dot(dnor) > T_MIN {
            outward_normal = -dnor;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = dnor;
            ni_over_nt = 1.0 / self.ref_idx;
        }
        let din = din.normalize();
        let dout = dout.normalize();
        match refract(din, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if vec_eq(&refracted, &dout) {
                    Some(RGBSpectrum::new(1.0, 1.0, 1.0))
                } else {
                    None
                }
            }
            None => {
                // reflect
                if (din + dout).dot(dnor).abs() < T_MIN {
                    Some(RGBSpectrum::new(1.0, 1.0, 1.0))
                } else {
                    None
                }
            }
        }
    }
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

    fn brdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> RGBSpectrum {
        if let Some(x) = self._brdf(din, dout, dnor) {
            x
        } else {
            BLACK
        }
    }

    fn pdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> f32 {
        if let Some(_) = self._brdf(din, dout, dnor) {
            1.0
        } else {
            ZERO
        }
    }

    fn attenuation(&self) -> RGBSpectrum {
        RGBSpectrum::new(1.0, 1.0, 1.0)
    }

    fn do_material(&self, r: &Ray, rec: &HitRecord, world: &mut World, depth: i32) -> RGBSpectrum {
        do_material_default(self, r, rec, world, depth)
    }
}

// the Cook-Torrance microfacet model
// see: https://zhuanlan.zhihu.com/p/304191958
fn ggx_distribution(n: Vec3, h: Vec3, alpha: f32) -> f32 {
    let noh = n.dot(h);
    let noh2 = noh.powi(2);
    let alpha2 = alpha.powi(2);
    alpha2 / (PI * (noh2 * (alpha2 - 1.0) + 1.0).powi(2))
}

fn ggx_schlick(nov: f32, k: f32) -> f32 {
    nov / (k + (1.0 - k) * nov)
}

fn ggx_partial_geometry_term(l: Vec3, v: Vec3, n: Vec3, alpha: f32) -> f32 {
    let nol = n.dot(l).abs();
    let nov = n.dot(v).abs();
    let r = alpha.sqrt();
    let k = (r + 1.0).powi(2) / 8.0;
    ggx_schlick(nol, k) * ggx_schlick(nov, k)
}

fn fresnel_schlick(din: Vec3, dnor: Vec3, f0: Vec3) -> Vec3 {
    f0.lerp(
        Vec3::new(1.0, 1.0, 1.0),
        (1.0 - dnor.dot(din).abs()).powi(2),
    )
}

fn half_vec(v1: &Vec3, v2: &Vec3) -> Vec3 {
    (v1 + v2).normalize()
}

#[derive(Clone)]
pub struct Microfacet {
    pub f0: RGBSpectrum,
    pub roughness: f32,
    pub metallic: f32,
    pub attenuation: RGBSpectrum,
}

/// return the basis of the local coordinate with the given vector be the y-axis
fn get_local_coordinate(v1: Vec3) -> (Vec3, Vec3, Vec3) {
    let v1 = v1.normalize();
    let v2 = if v1.x.abs() > v1.y.abs() {
        Vec3::new(-v1.z, 0.0, v1.x) / (v1.x.powi(2) + v1.z.powi(2)).sqrt()
    } else {
        Vec3::new(0.0, v1.z, -v1.y) / (v1.y.powi(2) + v1.z.powi(2)).sqrt()
    };
    let v3 = v1.cross(v2);
    (v2, v1, v3)
}

// see: https://computergraphics.stackexchange.com/questions/7656/importance-sampling-microfacet-ggx
// wg: geometry normal, wm: halfway vector
impl Material for Microfacet {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Ray> {
        assert!(false, "not supported");
        None
    }

    // see: https://schuttejoe.github.io/post/ggximportancesamplingpart1/
    fn scatter_d(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        // first sample a microfacet normal wm following NDF, with roughness
        // which is sampled in a local spherical coordinates
        let mut rng = rand::thread_rng();
        let r0 = rng.gen::<f32>();
        let r1 = rng.gen::<f32>();
        let a = self.roughness.powi(2);
        let a2 = a.powi(2);
        let cos_theta = ((1.0 - r0) / ((a2 - 1.0) * r0 + 1.0)).sqrt();
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let phi = 2.0 * PI * r1;

        // transform into Cartesian coordinates, with Y-up
        let x = sin_theta * phi.cos();
        let y = cos_theta;
        let z = sin_theta * phi.sin();

        // then transform into the global coordinate, with the given normal
        let (bx, by, bz) = get_local_coordinate(rec.normal);
        let wm = (x * bx + y * by + z * bz).normalize();

        // calculate the in direction with the reflection law
        let wo = r_in.d;
        let wi = reflect(wo, wm);
        Some(Ray { o: rec.p, d: wi })
    }

    fn brdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> RGBSpectrum {
        let wg = dnor.normalize();
        let wi = dout.normalize();
        let wo = -din.normalize();
        let wm = half_vec(&wi, &wo);
        let alpha = self.roughness.powi(2);

        if wi.dot(wg) <= 0.0 || wi.dot(wm) <= 0.0 {
            return BLACK;
        }

        let f0 = self.f0.lerp(self.attenuation, self.metallic);
        let f = fresnel_schlick(wi, wg, f0);
        let d = ggx_distribution(wg, wm, alpha);
        let g = ggx_partial_geometry_term(wi, wo, wg, alpha);
        let res = (f * d * g) / (4.0 * wg.dot(wi).abs() * wg.dot(wo).abs());
        mul_v(&self.attenuation, &res)
    }

    fn pdf(&self, din: Vec3, dout: Vec3, dnor: Vec3) -> f32 {
        let wg = dnor.normalize();
        let wo = -din.normalize();
        let wi = dout.normalize();
        let wm = half_vec(&wi, &wo);
        let a = self.roughness.powi(2);
        let a2 = a.powi(2);
        let cos_theta = wg.dot(wm).abs();
        let exp = (a2 - 1.0) * cos_theta.powi(2) + 1.0;
        let d = a2 / (PI * exp.powi(2));
        (d * cos_theta) / (4.0 * wo.dot(wm).abs())
    }

    // TODO: consider to remove this method
    fn attenuation(&self) -> RGBSpectrum {
        assert!(false);
        BLACK
    }

    // see: http://www.codinglabs.net/article_physically_based_rendering_cook_torrance.aspx
    fn do_material(&self, r: &Ray, rec: &HitRecord, world: &mut World, depth: i32) -> RGBSpectrum {
        // the specular part
        let specular = match self.scatter_d(&r, rec) {
            // for scatter case, the result is only dependent on the scattered ray
            Some(scattered) => {
                let li = path_trace_shader_mis(&scattered, world, depth + 1);
                let l_pdf = world.lights.pdf(&scattered);
                let b_pdf = self.pdf(r.d, scattered.d, rec.normal);
                // let li = mul_v(&self.attenuation, &li);
                mul_v(&li, &self.brdf(r.d, scattered.d, rec.normal)) / (l_pdf + b_pdf)
                // mul_v(&li, &m.brdf(r.d, scattered.d, rec.normal)) / (b_pdf)
            }
            None => BLACK,
        };

        // the diffuse part
        let (kd, diffuse) = {
            let mut d = unit_vec_on_sphere();
            if d.dot(rec.normal) < 0.0 {
                d = -d; // semisphere
            }
            let scattered = Ray { o: rec.p, d: d };

            let wi = scattered.d;
            let wg = rec.normal;
            let f0 = self.f0.lerp(self.attenuation, self.metallic);
            let f = fresnel_schlick(wi, wg, f0);
            let kd = (Vec3::new(1.0, 1.0, 1.0) - f) * (1.0 - self.metallic);

            // HACK: I'm not quite sure how to get the diffuse part right
            // calling the recursive shader again is far too heavy for "path tracing"
            // here just confine the depth to no more than `5` for a performance trade-off
            // This may work because the weight of diffuse is not much
            let depth = depth.max(34);
            let li = path_trace_shader_mis(&scattered, world, depth + 1);
            let l_pdf = world.lights.pdf(&scattered);
            let b_pdf = 1.0 / PI;
            (kd, mul_v(&li, &(self.attenuation / PI)) / (l_pdf + b_pdf))
        };
        let brdf = mul_v(&kd, &diffuse) + specular;

        let direct = match world.lights.visible_d(rec.p, rec.normal, &world.objects) {
            Some(LSampleRec { ray, radiance, p }) => {
                let b_pdf = self.pdf(r.d, ray.d, rec.normal);
                mul_v(&radiance, &self.brdf(r.d, ray.d, rec.normal)) / (p + b_pdf)
            }
            None => BLACK,
        };
        brdf + direct
        // direct
        // brdf
    }
}

fn do_material_default<M: Material>(
    m: &M,
    r: &Ray,
    rec: &HitRecord,
    world: &mut World,
    depth: i32,
) -> RGBSpectrum {
    let brdf = match m.scatter_d(&r, rec) {
        // for scatter case, the result is only dependent on the scattered ray
        Some(scattered) => {
            let li = path_trace_shader_mis(&scattered, world, depth + 1);
            let l_pdf = world.lights.pdf(&scattered);
            let b_pdf = m.pdf(r.d, scattered.d, rec.normal);
            mul_v(&li, &m.brdf(r.d, scattered.d, rec.normal)) / (l_pdf + b_pdf)
            // mul_v(&li, &m.brdf(r.d, scattered.d, rec.normal)) / (b_pdf)
        }
        None => BLACK,
    };

    let direct = match world.lights.visible_d(rec.p, rec.normal, &world.objects) {
        Some(LSampleRec { ray, radiance, p }) => {
            let b_pdf = m.pdf(r.d, ray.d, rec.normal);
            mul_v(&radiance, &m.brdf(r.d, ray.d, rec.normal)) / (p + b_pdf)
            // mul_v(&radiance, &m.brdf(r.d, ray.d, rec.normal)) / (p)
        }
        None => BLACK,
    };
    brdf + direct
    // direct
    // brdf
}
