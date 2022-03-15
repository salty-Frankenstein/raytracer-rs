const NX: i32 = 200;
const NY: i32 = 100;

use ray_tracer::geometry::*;
use ray_tracer::hitable::*;
use ray_tracer::ray::*;
use ray_tracer::*;

fn color(r: &Ray, world: &HitableList) -> Color {
    if let Some(rec) = world.hit(r, 0.0, 10000000.0) {
        0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0)
    } else {
        Color::new(0.8, 0.8, 0.8)
    }
}

fn main() {
    println!("P3\n{} {}\n255", NX, NY);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);

    let world = HitableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            // Box::new(Sphere {
            //     center: Vec3::new(0.0, -100.5, -1.0),
            //     radius: 100.0,
            // }),
        ],
    };

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f32 / NX as f32;
            let v = j as f32 / NY as f32;
            let r = Ray {
                o: origin,
                d: lower_left_corner + u * horizontal + v * vertical,
            };
            let col = color(&r, &world);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
