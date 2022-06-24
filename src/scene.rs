use crate::camera::*;
use crate::geometry::*;
use crate::hitable::*;
use crate::light::*;
use crate::material::*;
use crate::mesh::*;
use crate::obj_loader::*;
use crate::sampler::*;
use crate::shader::*;
use crate::*;
use cgmath::prelude::*;
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

fn make_chess_board(vertex: (Pt3, Pt3, Pt3, Pt3), num: i32) -> Vec<Triangle> {
    let ori = vertex.0.to_vec();
    let vi = (vertex.1 - vertex.0) / num as f32;
    let vj = (vertex.3 - vertex.0) / num as f32;
    let mut list = Vec::new();
    for i in 1..num + 1 {
        for j in 1..num + 1 {
            let p00 = Pt3::from_vec(ori + (i - 1) as f32 * vi + (j - 1) as f32 * vj);
            let p01 = Pt3::from_vec(ori + (i - 1) as f32 * vi + j as f32 * vj);
            let p10 = Pt3::from_vec(ori + i as f32 * vi + (j - 1) as f32 * vj);
            let p11 = Pt3::from_vec(ori + i as f32 * vi + j as f32 * vj);
            let color = if (i + j) % 2 == 0 { BLACK } else { WHITE };
            let (t1, t2) = make_square((p00, p10, p11, p01), color);
            list.push(t1);
            list.push(t2);
        }
    }
    list
}

impl Scene {
    pub fn cornell_box(sampler_kind: SamplerKind) -> obj::ObjResult<Scene> {
        let mut pyramid = load_obj_file(
            String::from("./input/pyramid.obj"),
            // Dielectric {ref_idx: 1.8}
            // Metal {
            //     albedo: Vec3::new(1.0, 1.0, 1.0),
            // },
            Microfacet {
                f0: RGBSpectrum::new(0.98, 0.98, 0.98),
                roughness: 0.6,
                metallic: 0.5,
                attenuation: RGBSpectrum::new(0.8, 0.8, 0.8),
            },
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
            // Metal {
            //     albedo: Vec3::new(1.0, 0.7, 0.9),
            // },
            Microfacet {
                f0: RGBSpectrum::new(0.98, 0.98, 0.98),
                roughness: 0.2,
                metallic: 0.9,
                attenuation: RGBSpectrum::new(1.0, 0.7, 0.9),
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
        utah.displacement(Vec3::new(0.1, -0.507, -1.9));
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
        let (t1, t2) = make_square((v1, v2, v3, v4), RGBSpectrum::new(0.9, 0.9, 0.9));
        let (t3, t4) = make_square((v5, v8, v7, v6), RGBSpectrum::new(0.9, 0.9, 0.9));
        let (t5, t6) = make_square((v4, v3, v7, v8), RGBSpectrum::new(0.9, 0.9, 0.9));
        let (t7, t8) = make_square((v1, v4, v8, v5), RGBSpectrum::new(0.7, 0.0, 0.0));
        let (t9, t10) = make_square((v3, v2, v6, v7), RGBSpectrum::new(0.0, 0.4, 0.0));
        let chess_board = make_chess_board((v4, v3, v7, v8), 12);
        let acc = FromFaceList::from_face_list(&chess_board);
        let mut chess_board_mesh = FastMesh {
            face_list: chess_board,
            acc_structure: acc,
        };
        chess_board_mesh.rotate(0.01, 0.0, 0.0); // avoid abnormal triangles

        let (t11, t12) = make_square(
            (
                Pt3::new(-0.22, 1.0, -2.22),
                Pt3::new(0.22, 1.0, -2.22),
                Pt3::new(0.22, 1.0, -1.78),
                Pt3::new(-0.22, 1.0, -1.78),
            ),
            BLACK,
        );
        let square = vec![t11, t12];
        let square_acc = FromFaceList::from_face_list(&square);
        let square_mesh = NaiveMesh {
            face_list: square,
            acc_structure: square_acc,
        };
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
                        // Box::new(chess_board_mesh),
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
                            // mat: Rc::new(Metal {
                            //     albedo: RGBSpectrum::new(0.9, 0.7, 0.4),
                            // }),
                            mat: Rc::new(Microfacet {
                                f0: RGBSpectrum::new(0.98, 0.98, 0.98),
                                roughness: 0.13,
                                metallic: 0.9,
                                attenuation: RGBSpectrum::new(0.9, 0.7, 0.4),
                            }),
                        }),
                        Box::new(Sphere {
                            center: Vec3::new(-0.34, 0.38, -2.0),
                            radius: 0.25,
                            // mat: Rc::new(Dielectric { ref_idx: 1.5 }),
                            // mat: Rc::new(Metal {
                            //     albedo: RGBSpectrum::new(0.4, 0.7, 0.9),
                            // }),
                            mat: Rc::new(Microfacet {
                                f0: RGBSpectrum::new(0.18, 0.18, 0.18),
                                roughness: 0.3,
                                metallic: 0.2,
                                attenuation: RGBSpectrum::new(0.4, 0.7, 0.9),
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
                        // Box::new(DiskLight::new(
                        //     Pt3::new(0.0, 1.0, -2.0),
                        //     0.4,
                        //     RGBSpectrum::new(0.9, 0.64, 0.48) * 6.0,
                        //     sampler_kind,
                        // )),
                        // Box::new(DiskLight::new(
                        //     Pt3::new(-0.45, 1.0, -2.25),
                        //     0.4,
                        //     RGBSpectrum::new(0.9, 0.0, 0.48) * 6.0,
                        //     sampler_kind,
                        // )),
                        // Box::new(DiskLight::new(
                        //     Pt3::new(0.45, 1.0, -1.75),
                        //     0.4,
                        //     RGBSpectrum::new(0.0, 0.63, 0.48) * 6.0,
                        //     sampler_kind,
                        // )),
                        Box::new(PolygonLight::new(
                            square_mesh,
                            RGBSpectrum::new(0.9, 0.64, 0.28) * 22.0,
                        )),
                    ],
                },
            },
        };
        Ok(s)
    }

    pub fn blue_noise_test() -> Scene {
        let a = (
            Pt3::new(-1.0, 0.0, -1.0),
            Pt3::new(1.0, 0.0, -1.0),
            Pt3::new(1.0, 0.0, -3.0),
            Pt3::new(-1.0, 0.0, -3.0),
        );
        let chess_board = make_chess_board(a, 24);
        let acc = FromFaceList::from_face_list(&chess_board);
        let mut chess_board_mesh = FastMesh {
            face_list: chess_board,
            acc_structure: acc,
        };
        chess_board_mesh.displacement(Vec3::new(0.0, 0.0, 2.0));
        chess_board_mesh.scale(5.0);
        chess_board_mesh.rotate(10.0, 30.0, 5.1);
        chess_board_mesh.displacement(Vec3::new(0.0, -0.5, -7.0));
        let s = Scene {
            cam: Camera::new(
                Pt3::new(0.0, 0.0, 2.0),
                Pt3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                NX as f32 / NY as f32,
            ),
            world: World {
                objects: HitableList {
                    list: vec![Box::new(chess_board_mesh)],
                },
                lights: LightList {
                    list: vec![
                        Box::new(PointLight {
                            origin: Pt3::new(0.0, 2.0, -5.0),
                            spectrum: RGBSpectrum::new(0.9, 0.64, 0.48) * 10.8,
                        }),
                        // Box::new(DiskLight {
                        //     origin: Pt3::new(0.0, 2.0, -5.0),
                        //     radius: 0.4,
                        //     spectrum: RGBSpectrum::new(0.9, 0.64, 0.48) * 10.0,
                        // }),
                    ],
                },
            },
        };
        s
    }

    pub fn light_test(sampler_kind: SamplerKind) -> Scene {
        let ceiling = -0.5;
        let ground = -0.57;
        let brightness = 8.0;
        let sq = (
            Pt3::new(-1.0, ground, -1.0),
            Pt3::new(1.0, ground, -1.0),
            Pt3::new(1.0, ground, -3.0),
            Pt3::new(-1.0, ground, -3.0),
        );
        let (t1, t2) = make_square(sq, RGBSpectrum::new(1.0, 1.0, 1.0));
        // let dummy_mat = Dielectric { ref_idx: 0.0 };
        let dummy_mat = Rc::new(Dielectric { ref_idx: 0.0 });

        let triangle = vec![Triangle {
            vertex: (
                Pt3::new(-0.15, ceiling, -2.15),
                Pt3::new(0.15, ceiling, -2.15),
                Pt3::new(0.0, ceiling, -1.85),
            ),
            mat: dummy_mat.clone(),
        }];
        let triangle_acc = FromFaceList::from_face_list(&triangle);
        let triangle_mesh = NaiveMesh {
            face_list: triangle,
            acc_structure: triangle_acc,
        };

        let interval = 0.6;
        let (t3, t4) = make_square(
            (
                Pt3::new(-0.1 + interval, ceiling, -1.9),
                Pt3::new(0.1 + interval, ceiling, -1.9),
                Pt3::new(0.1 + interval, ceiling, -2.1),
                Pt3::new(-0.1 + interval, ceiling, -2.1),
            ),
            BLACK,
        );
        let square = vec![t3, t4];
        let square_acc = FromFaceList::from_face_list(&square);
        let square_mesh = NaiveMesh {
            face_list: square,
            acc_structure: square_acc,
        };

        let hexagon = vec![
            Triangle {
                vertex: (
                    Pt3::new(-0.1 - interval, ceiling, -2.17),
                    Pt3::new(0.1 - interval, ceiling, -2.17),
                    Pt3::new(0.0 - interval, ceiling, -2.0),
                ),
                mat: dummy_mat.clone(),
            },
            Triangle {
                vertex: (
                    Pt3::new(0.1 - interval, ceiling, -2.17),
                    Pt3::new(0.2 - interval, ceiling, -2.0),
                    Pt3::new(0.0 - interval, ceiling, -2.0),
                ),
                mat: dummy_mat.clone(),
            },
            Triangle {
                vertex: (
                    Pt3::new(-0.2 - interval, ceiling, -2.0),
                    Pt3::new(-0.1 - interval, ceiling, -2.17),
                    Pt3::new(0.0 - interval, ceiling, -2.0),
                ),
                mat: dummy_mat.clone(),
            },
            Triangle {
                vertex: (
                    Pt3::new(0.1 - interval, ceiling, -1.83),
                    Pt3::new(-0.1 - interval, ceiling, -1.83),
                    Pt3::new(0.0 - interval, ceiling, -2.0),
                ),
                mat: dummy_mat.clone(),
            },
            Triangle {
                vertex: (
                    Pt3::new(0.2 - interval, ceiling, -2.0),
                    Pt3::new(0.1 - interval, ceiling, -1.83),
                    Pt3::new(0.0 - interval, ceiling, -2.0),
                ),
                mat: dummy_mat.clone(),
            },
            Triangle {
                vertex: (
                    Pt3::new(-0.1 - interval, ceiling, -1.83),
                    Pt3::new(-0.2 - interval, ceiling, -2.0),
                    Pt3::new(0.0 - interval, ceiling, -2.0),
                ),
                mat: dummy_mat.clone(),
            },
        ];
        let hexagon_acc = FromFaceList::from_face_list(&hexagon);
        let hexagon_mesh = NaiveMesh {
            face_list: hexagon,
            acc_structure: hexagon_acc,
        };

        Scene {
            cam: Camera::new(
                Pt3::new(0.0, 1.5, 3.0),
                Pt3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                NX as f32 / NY as f32,
            ),
            world: World {
                objects: HitableList {
                    list: vec![Box::new(t1), Box::new(t2)],
                },
                lights: LightList {
                    list: vec![
                        // Box::new(DiskLight::new(
                        //     Pt3::new(0.6, ceiling, -2.0),
                        //     0.1,
                        //     RGBSpectrum::new(0.9, 0.64, 0.48) * brightness,
                        //     sampler_kind,
                        // )),
                        // Box::new(DiskLight::new(
                        //     Pt3::new(0.0, ceiling, -2.0),
                        //     0.1,
                        //     RGBSpectrum::new(0.9, 0.64, 0.48) * brightness,
                        //     sampler_kind,
                        // )),
                        // Box::new(DiskLight::new(
                        //     Pt3::new(-0.6, ceiling, -2.0),
                        //     0.1,
                        //     RGBSpectrum::new(0.9, 0.64, 0.48) * brightness,
                        //     sampler_kind,
                        // )),
                        Box::new(PolygonLight::new(
                            triangle_mesh,
                            RGBSpectrum::new(0.9, 0.64, 0.48) * brightness,
                        )),
                        Box::new(PolygonLight::new(
                            square_mesh,
                            RGBSpectrum::new(0.9, 0.64, 0.48) * brightness,
                        )),
                        Box::new(PolygonLight::new(
                            hexagon_mesh,
                            RGBSpectrum::new(0.9, 0.64, 0.48) * brightness,
                        )),
                    ],
                },
            },
        }
    }
}
