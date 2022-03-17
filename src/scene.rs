use crate::camera::*;
use crate::obj_loader::*;
use crate::shader::*;
use crate::*;
use crate::hitable::*;
use crate::light::*;
use crate::geometry::*;
use crate::material::*;
use std::rc::Rc;

pub struct Scene {
    pub cam: Camera,
    pub world: World,
}

impl Scene {
    pub fn test_scene() -> obj::ObjResult<Scene> {
        let mut cube = load_obj_file(String::from("./input/cube.obj"))?;
        cube.scale(0.05);
        cube.rotate(30.0, 40.0, 30.0);
        cube.displacement(Vec3::new(-0.2, -0.0, -1.0));

        let mut miku = load_obj_file(String::from("./input/miku.obj"))?;
        miku.scale(0.01);
        miku.rotate(-90.0, 0.0, -5.0);
        miku.displacement(Vec3::new(1.0, -0.5, -1.5));

        let s = Scene {
            cam: Camera::new(
                Pt3::new(0.0, 0.0, 0.0),
                Pt3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                NX as f32 / NY as f32,
            ),
            world: World {
                objects: HitableList {
                    list: vec![
                        Box::new(miku),
                        Box::new(cube),
                        Box::new(Sphere {
                            center: Vec3::new(-0.5, 0.0, -1.0),
                            radius: 0.2,
                            mat: Rc::new(Metal {
                                albedo: RGBSpectrum::new(1.0, 1.0, 1.0),
                            }),
                        }),
                        Box::new(Sphere {
                            center: Vec3::new(0.0, -100.5, -1.0),
                            radius: 100.0,
                            mat: Rc::new(Metal {
                                albedo: RGBSpectrum::new(0.9, 0.5, 0.7),
                            }),
                        }),
                    ],
                },
                lights: LightList {
                    list: vec![
                        Box::new(PointLight {
                            origin: Pt3::new(1.0, 1.0, 2.0),
                            spectrum: RGBSpectrum::new(0.5, 0.5, 0.5),
                        }),
                        // Box::new(PointLight {
                        //     origin: Pt3::new(1.0, 1.0, 2.0),
                        //     spectrum: RGBSpectrum::new(0.5, 0.0, 0.0),
                        // }),
                        // Box::new(PointLight {
                        //     origin: Pt3::new(-1.0, 1.0, 2.0),
                        //     spectrum: RGBSpectrum::new(0.0, 0.0, 0.5),
                        // }),
                        // Box::new(PointLight {
                        //     origin: Pt3::new(0.0, 3.0, 2.0),
                        //     spectrum: RGBSpectrum::new(0.0, 0.5, 0.0),
                        // }),
                    ],
                },
            },
        };
        Ok(s)
    }
}
