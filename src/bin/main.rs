use ray_tracer::geometry::*;
use ray_tracer::hitable::*;
use ray_tracer::light::*;
use ray_tracer::material::*;
use ray_tracer::obj_loader::*;
use ray_tracer::ray::*;
use ray_tracer::shader::*;
use ray_tracer::*;
use std::fs::File;
use std::io::*;
use std::rc::Rc;

const NX: i32 = 400;
const NY: i32 = 400;

fn main() -> obj::ObjResult<()> {
    let origin = Pt3::new(0.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let horizontal = Vec3::new(2.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-1.0, -1.0, -1.0);

    let mut cube = load_obj_file(String::from("./input/cube.obj"))?;
    cube.scale(0.05);
    cube.rotate(30.0, 40.0, 30.0);
    cube.displacement(Vec3::new(-0.2, -0.0, -1.0));

    let mut miku = load_obj_file(String::from("./input/miku.obj"))?;
    miku.scale(0.01);
    miku.rotate(-90.0, 0.0, -5.0);
    miku.displacement(Vec3::new(1.0, -0.5, -1.5));

    let world = World {
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
                    spectrum: RGBSpectrum::new(0.7, 0.7, 0.7),
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
    };

    let mut output = File::create("./output/out.ppm")?;

    writeln!(&mut output, "P3\n{} {}\n255", NX, NY)?;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f32 / NX as f32;
            let v = j as f32 / NY as f32;
            let r = Ray {
                o: origin,
                d: lower_left_corner + u * horizontal + v * vertical,
            };
            // let col = normal_shade(&r, &world);
            let col = trace_shader(&r, &world, 0);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            writeln!(&mut output, "{} {} {}", ir, ig, ib)?;
        }
        if j % 10 == 0 {
            println!("Now rendering: {}%", ((NY - j) * 100) as f32 / NY as f32);
        }
    }
    println!("Finished.");
    Ok(())
}
