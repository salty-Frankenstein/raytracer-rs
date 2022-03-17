use rand::prelude::*;
use ray_tracer::scene::Scene;
use ray_tracer::shader::*;
use ray_tracer::*;
use std::fs::File;
use std::io::*;
use std::time::*;

fn main() -> obj::ObjResult<()> {
    let now = Instant::now();
    // let scene = Scene::test_scene()?;
    let scene = Scene::cornell_box()?;
    let mut output = File::create("./output/out.ppm")?;

    writeln!(&mut output, "P3\n{} {}\n255", NX, NY)?;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let mut rng = rand::thread_rng();
                let u = (i as f32 + rng.gen::<f32>()) / NX as f32;
                let v = (j as f32 + rng.gen::<f32>()) / NY as f32;

                let r = scene.cam.get_ray(u, v);
                // col += normal_shade(&r, &scene.world);
                col += trace_shader(&r, &scene.world, 0);
            }
            col = &col / NS as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            writeln!(&mut output, "{} {} {}", ir, ig, ib)?;
        }
        if j % 5 == 0 {
            println!(
                "Now rendering: {}%, {} seconds elapsed",
                ((NY - j) * 100) as f32 / NY as f32,
                now.elapsed().as_secs()
            );
        }
    }
    println!("Finished.");
    Ok(())
}
