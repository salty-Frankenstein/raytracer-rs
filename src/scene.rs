use crate::camera::*;
use crate::geometry::*;
use crate::hitable::*;
use crate::light::*;
use crate::material::*;
use crate::obj_loader::*;
use crate::shader::*;
use crate::*;
use std::rc::Rc;

pub struct Scene {
    pub cam: Camera,
    pub world: World,
}

fn make_square(vertex: (Pt3, Pt3, Pt3, Pt3), albedo: RGBSpectrum) -> (Triangle, Triangle) {
    let t1 = Triangle {
        vertex: (vertex.0, vertex.1, vertex.2),
        mat: Rc::new(Diffuse { albedo: albedo }),
    };
    let t2 = Triangle {
        vertex: (vertex.0, vertex.2, vertex.3),
        mat: Rc::new(Diffuse { albedo: albedo }),
    };
    (t1, t2)
}

impl Scene {
    pub fn cornell_box() -> obj::ObjResult<Scene> {
        let mut pyramid = load_obj_file(
            String::from("./input/pyramid.obj"),
            Dielectric {ref_idx: 1.8}
            // Metal {
            //     albedo: Vec3::new(1.0, 1.0, 1.0),
            // },
        )?;
        pyramid.scale(8.0);
        pyramid.rotate(0.0, -15.0, 0.0);
        pyramid.displacement(Vec3::new(0.55, 0.1, -2.6));

        // let mut miku = load_obj_file(String::from("./input/miku.obj"), true)?;
        // miku.scale(0.01);
        // miku.rotate(-90.0, 0.0, -35.0);
        // miku.displacement(Vec3::new(0.5, -0.97, -1.3));

        let mut miku2 = load_obj_file(
            String::from("./input/.miku2.obj"),
            // Dielectric{ref_idx: 1.8}
            Metal {
                albedo: Vec3::new(1.0, 0.7, 0.9),
            },
        )?;
        miku2.transform(0.008, Vec3::new(0.6, -1.0, -1.3), -90.0, 0.0, -35.0);

        let mut miku3 = load_obj_file(
            String::from("./input/.miku3.obj"),
            Dielectric{ref_idx: 1.8}
            // Metal {
            //     albedo: Vec3::new(1.0, 1.0, 1.0),
            // },
        )?;
        miku3.transform(0.06, Vec3::new(-0.5, -1.0, -1.5), -90.0, 0.0, 35.0);

        let mut utah = load_obj_file(
            String::from("./input/utah.obj"),
            // Dielectric { ref_idx: 1.8 },
            Diffuse {
                albedo: Vec3::new(1.0, 1.0, 1.0),
            },
        )?;
        utah.scale(0.1);
        utah.displacement(Vec3::new(0.1, -0.5, -1.9));
        let (v1, v2, v3, v4) = (
            Pt3::new(-1.0, -1.0, -1.0),
            Pt3::new(1.0, -1.0, -1.0),
            Pt3::new(1.0, -1.0, -3.0),
            Pt3::new(-1.0, -1.0, -3.0),
        );
        let (v5, v6, v7, v8) = (
            Pt3::new(-1.0, 1.0, -1.0),
            Pt3::new(1.0, 1.0, -1.0),
            Pt3::new(1.0, 1.0, -3.0),
            Pt3::new(-1.0, 1.0, -3.0),
        );
        let (t1, t2) = make_square((v1, v2, v3, v4), RGBSpectrum::new(0.8, 0.8, 0.8));
        let (t3, t4) = make_square((v5, v8, v7, v6), RGBSpectrum::new(0.8, 0.8, 0.8));
        let (t5, t6) = make_square((v4, v3, v7, v8), RGBSpectrum::new(0.8, 0.8, 0.8));
        let (t7, t8) = make_square((v1, v4, v8, v5), RGBSpectrum::new(0.8, 0.0, 0.0));
        let (t9, t10) = make_square((v3, v2, v6, v7), RGBSpectrum::new(0.0, 0.8, 0.0));
        let s = Scene {
            cam: Camera::new(
                // Pt3::new(0.0, 10.0, 0.0),
                // Pt3::new(0.0, -1.0, 0.0),
                // Vec3::new(0.0, 0.0, -1.0),
                Pt3::new(0.0, 0.0, 2.0),
                Pt3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                NX as f32 / NY as f32,
            ),
            world: World {
                objects: HitableList {
                    list: vec![
                        Box::new(utah),
                        // Box::new(miku),
                        Box::new(miku2),
                        Box::new(miku3),
                        Box::new(pyramid),
                        Box::new(t1),
                        Box::new(t2),
                        Box::new(t3),
                        Box::new(t4),
                        Box::new(t5),
                        Box::new(t6),
                        Box::new(t7),
                        Box::new(t8),
                        Box::new(t9),
                        Box::new(t10),
                        Box::new(Cylinder {
                            center_x: 0.1,
                            center_z: -1.95,
                            radius: 0.4,
                            y_max: -0.5,
                            y_min: -1.0,
                            mat: Rc::new(Metal {
                                albedo: RGBSpectrum::new(0.9, 0.7, 0.4),
                            }),
                        }),
                        Box::new(Sphere {
                            center: Vec3::new(-0.34, 0.38, -2.0),
                            radius: 0.25,
                            // mat: Rc::new(Dielectric { ref_idx: 1.5 }),
                            mat: Rc::new(Metal {
                                albedo: RGBSpectrum::new(0.4, 0.7, 0.9),
                            }),
                        }),
                    ],
                },
                lights: LightList {
                    list: vec![
                        // Box::new(PointLight {
                        //     origin: Pt3::new(0.0, 0.0, 0.0),
                        //     spectrum: RGBSpectrum::new(0.9, 0.64, 0.48) * 1.8,
                        // }),
                        // Box::new(DiskLight {
                        //     origin: Pt3::new(0.0, 0.0, 0.0),
                        //     radius: 1.0,
                        //     spectrum: RGBSpectrum::new(0.9, 0.64, 0.48) * 1.8,
                        // }),
                        // Box::new(PointLight {
                        //     origin: Pt3::new(0.0, 1.0, -2.0),
                        //     spectrum: RGBSpectrum::new(0.9, 0.64, 0.48) * 1.8,
                        // }),
                        Box::new(DiskLight {
                            origin: Pt3::new(0.0, 1.0, -2.0),
                            radius: 0.4,
                            spectrum: RGBSpectrum::new(0.9, 0.64, 0.48) * 2.0,
                        }),
                        // Box::new(DiskLight {
                        //     origin: Pt3::new(-0.45, 1.0, -2.25),
                        //     radius: 0.4,
                        //     spectrum: RGBSpectrum::new(0.9, 0.0, 0.48) * 2.0,
                        // }),
                        // Box::new(DiskLight {
                        //     origin: Pt3::new(0.45, 1.0, -1.75),
                        //     radius: 0.4,
                        //     spectrum: RGBSpectrum::new(0.0, 0.63, 0.48) * 2.0,
                        // }),
                    ],
                },
            },
        };
        Ok(s)
    }
}
