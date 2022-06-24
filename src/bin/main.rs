use ray_tracer::sampler::*;
use ray_tracer::scene::Scene;
use ray_tracer::shader::*;
use ray_tracer::*;
use std::env;
use std::fs::File;
use std::io::*;
use std::time::*;

fn main() -> obj::ObjResult<()> {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let parse_to_io_err = |_| Error::new(ErrorKind::Other, "parse int error");
    // let pixel_samper = args[1].parse::<i32>().map_err(parse_to_io_err).and_then(SamplerKind::from_int)?;
    let light_samper = args[1]
        .parse::<i32>()
        .map_err(parse_to_io_err)
        .and_then(SamplerKind::from_int)?;

    // let mut scene = Scene::light_test(light_samper);
    let mut scene = Scene::cornell_box(light_samper)?;
    // let scene = Scene::blue_noise_test();
    let mut output = File::create("./output/out.ppm")?;

    writeln!(&mut output, "P3\n{} {}\n255", NX, NY)?;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            let mut sampler = BlueNoiseSampler::new(1.0, NS, false);
            // let mut sampler = JitteredSampler::new(1.0, NS);
            // let mut sampler = UniformSampler::new(1.0, NS);
            // let mut sampler = WhiteNoiseSampler::new(1.0, NS);
            while let Some((a, b)) = sampler.sample() {
                let u = (i as f32 + a) / NX as f32;
                let v = (j as f32 + b) / NY as f32;

                let r = scene.cam.get_ray(u, v);
                // col += normal_shader(&r, &scene.world);
                // col += whitted_trace_shader(&r, &mut scene.world, 0);
                // col += path_trace_shader(&r, &mut scene.world, 0);
                col += path_trace_shader_mis(&r, &mut scene.world, 0);
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
