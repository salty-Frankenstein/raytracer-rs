use crate::geometry::*;
use crate::hitable::*;
use crate::mesh::*;
use crate::*;
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use crate::material::*;
use std::rc::Rc;

/// load an obj file, parse into a hitable
pub fn load_obj_file(path: String) -> obj::ObjResult<Mesh> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let model: Obj<obj::Position> = load_obj(input)?;
    let mut i = 0;
    let mut list: Vec<Triangle> = Vec::new();
    while i < model.indices.len() {
        let idx0 = model.indices[i] as usize;
        let idx1 = model.indices[i + 1] as usize;
        let idx2 = model.indices[i + 2] as usize;
        let p0 = model.vertices[idx0].position;
        let p1 = model.vertices[idx1].position;
        let p2 = model.vertices[idx2].position;

        list.push(Triangle{
            vertex: (
            Pt3::new(p0[0], p0[1], p0[2]),
            Pt3::new(p1[0], p1[1], p1[2]),
            Pt3::new(p2[0], p2[1], p2[2])),
            // TODO: into reference
            mat: Rc::new(Metal{albedo: Vec3::new(0.4,0.7,0.9)})
    });
        i = i + 3;
    }
    Ok(Mesh { face_list: list })
}
