use crate::hitable::*;
use crate::light::*;
use crate::ray::*;
use crate::*;
use cgmath::prelude::*;

/// shade with the normal vector of the hitting surface
pub fn normal_shade(r: &Ray, world: &World) -> RGBSpectrum {
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

/// ray tracing shader
pub fn trace_shader(r: &Ray, world: &World, depth: i32) -> RGBSpectrum {
    if depth > 40 {
        return BLACK;
    }

    if let Some(direct) = world.lights.hit(r) {
        return direct;
    }
    match world.objects.hit(r, T_MIN, T_MAX) {
        // intersect, then trace
        Some(rec) => {
            // calculate the shadow ray
            let recr = &rec.clone();
            match rec.mat {
                Some(m) => match m.scatter(&r, recr) {
                    // for scatter case, the result is only dependent on the scattered ray
                    Some(scattered) => {
                        let t = trace_shader(&scattered, world, depth + 1);
                        mul_v(&t, &m.attenuation())
                    }
                    // for diffuse case, check visibility
                    None => match world.lights.visible(rec.p, rec.normal, &world.objects) {
                        Some(direct) => mul_v(&direct, &m.attenuation()),
                        None => BLACK,
                    },
                },
                None => panic!("no material"),
            }
        }
        None => BLACK,
    }
}
