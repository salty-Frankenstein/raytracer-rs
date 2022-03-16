use crate::hitable::*;
use crate::light::*;
use crate::ray::*;
use crate::*;

/// shade with the normal vector of the hitting surface
pub fn normal_shade(r: &Ray, world: &impl Hitable) -> RGBSpectrum {
    if let Some(rec) = world.hit(r, T_MIN, T_MAX) {
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
pub fn trace_shader(r: &Ray, world: &World) -> RGBSpectrum {
    // TODO: direct to light source
    match world.objects.hit(r, T_MIN, T_MAX) {
        // intersect, then trace
        Some(rec) => {
            // calculate the shadow ray
            match world.lights.visible(rec.p, &world.objects) {
                Some(r) => r,
                None => BLACK,
            }
        }
        None => BLACK,
    }
}
