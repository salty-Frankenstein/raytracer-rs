use crate::hitable::*;
use crate::light::*;
use crate::ray::*;
use crate::*;

/// shade with the normal vector of the hitting surface
pub fn normal_shader(r: &Ray, world: &World) -> RGBSpectrum {
    if let Some(rec) = world.objects.hit(r, T_MIN, T_MAX) {
        0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0)
    } else {
        RGBSpectrum::new(0.8, 0.8, 0.8)
    }
}

/// a scene world is consist of a list of objects and a list of lights
pub struct World {
    pub objects: HitableList,
    pub lights: LightList,
}

/// whitted-style ray tracing shader
pub fn whitted_trace_shader(r: &Ray, world: &mut World, depth: i32) -> RGBSpectrum {
    if depth > 40 {
        return BLACK;
    }

    // hit the objects and the lights, deal with the closer one
    match (world.objects.hit(r, T_MIN, T_MAX), world.lights.hit(r)) {
        (Some(rec), lrec) => {
            if lrec.is_none() || lrec.unwrap().1 > rec.t {
                let recr = &rec.clone();
                match rec.mat {
                    Some(m) => match m.scatter(&r, recr) {
                        // for scatter case, the result is only dependent on the scattered ray
                        Some(scattered) => {
                            let t = whitted_trace_shader(&scattered, world, depth + 1);
                            mul_v(&t, &m.attenuation())
                        }
                        // for diffuse case, check visibility
                        // calculate the shadow ray
                        None => match world.lights.visible(rec.p, rec.normal, &world.objects) {
                            Some(direct) => mul_v(&direct, &m.attenuation()),
                            None => BLACK,
                        },
                    },
                    None => panic!("no material"),
                }
            } else {
                lrec.unwrap().0
            }
        }
        (None, Some((direct, _))) => direct,
        (None, None) => BLACK,
    }
}

/// distrubuted ray tracing shader, performing path tracing
pub fn path_trace_shader(r: &Ray, world: &mut World, depth: i32) -> RGBSpectrum {
    if depth > 40 {
        return BLACK;
    }

    // hit the objects and the lights, deal with the closer one
    match (world.objects.hit(r, T_MIN, T_MAX), world.lights.hit(r)) {
        (Some(rec), lrec) => {
            if lrec.is_none() || lrec.unwrap().1 > rec.t {
                let recr = &rec.clone();
                match rec.mat {
                    Some(m) => {
                        let scattered = match m.scatter_d(&r, recr) {
                            // for scatter case, the result is only dependent on the scattered ray
                            Some(scattered) => {
                                let t = path_trace_shader(&scattered, world, depth + 1);
                                mul_v(&t, &m.attenuation())
                            }
                            None => BLACK,
                        };

                        scattered
                    }
                    None => panic!("no material"),
                }
            } else {
                lrec.unwrap().0
            }
        }
        (None, Some((direct, _))) => direct,
        (None, None) => BLACK,
    }
}

/// with MIS sampling
pub fn path_trace_shader_mis(r: &Ray, world: &mut World, depth: i32) -> RGBSpectrum {
    if depth > 40 {
        return BLACK;
    }

    // hit the objects and the lights, deal with the closer one
    match (world.objects.hit(r, T_MIN, T_MAX), world.lights.hit(r)) {
        (Some(rec), lrec) => {
            if lrec.is_none() || lrec.unwrap().1 > rec.t {
                let recr = &rec.clone();
                match rec.mat {
                    Some(m) => {
                        let brdf = match m.scatter_d(&r, recr) {
                            // for scatter case, the result is only dependent on the scattered ray
                            Some(scattered) => {
                                let li = path_trace_shader(&scattered, world, depth + 1);
                                let l_pdf = world.lights.pdf(&scattered);
                                let b_pdf = m.pdf(scattered.d, r.d, rec.normal);
                                mul_v(&li, &m.brdf(scattered.d, r.d, rec.normal)) / (l_pdf + b_pdf)
                            }
                            None => BLACK,
                        };

                        let direct = match world.lights.visible_d(rec.p, rec.normal, &world.objects)
                        {
                            Some(LSampleRec { ray, radiance, p }) => {
                                let b_pdf = m.pdf(ray.d, r.d, rec.normal);
                                mul_v(&radiance, &m.brdf(ray.d, r.d, rec.normal)) / (p + b_pdf)
                            }
                            None => BLACK,
                        };
                        brdf + direct
                        // direct
                        // brdf
                    }
                    None => panic!("no material"),
                }
            } else {
                lrec.unwrap().0
            }
        }
        (None, Some((direct, _))) => direct,
        (None, None) => BLACK,
    }
}
