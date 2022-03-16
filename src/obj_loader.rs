use crate::geometry::*;
use crate::hitable::*;
use crate::Vec3;
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

pub fn load_obj_file(path: String) -> obj::ObjResult<HitableList> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let model: Obj<obj::Position> = load_obj(input)?;

    let mut i = 0;
    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    while i < model.indices.len() {
        let idx0 = model.indices[i] as usize;
        let idx1 = model.indices[i + 1] as usize;
        let idx2 = model.indices[i + 2] as usize;
        let p0 = model.vertices[idx0].position;
        let p1 = model.vertices[idx1].position;
        let p2 = model.vertices[idx2].position;

        list.push(Box::new(Triangle(
            Vec3::new(p0[0], p0[1], p0[2] - 150.0),
            Vec3::new(p1[0], p1[1], p1[2] - 150.0),
            Vec3::new(p2[0], p2[1], p2[2] - 150.0),
        )));
        i = i + 3;
    }

    Ok(HitableList { list: list })
}
