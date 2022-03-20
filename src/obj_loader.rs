use crate::geometry::*;
use crate::material::*;
use crate::mesh::*;
use crate::*;
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

/// choosing mesh implementations
// type MeshT = NaiveMesh;
// type MeshT = BoxMesh;
type MeshT = FastMesh;

/// load an obj file, parse into a hitable
pub fn load_obj_file(path: String, mat: impl Material + 'static + Clone) -> obj::ObjResult<MeshT> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let model: Obj<obj::Position, usize> = load_obj(input)?;
    let mut i = 0;
    let mut list: Vec<Triangle> = Vec::new();
    while i < model.indices.len() {
        let idx0 = model.indices[i];
        let idx1 = model.indices[i + 1];
        let idx2 = model.indices[i + 2];
        let p0 = model.vertices[idx0].position;
        let p1 = model.vertices[idx1].position;
        let p2 = model.vertices[idx2].position;

        list.push(Triangle {
            vertex: (
                Pt3::new(p0[0], p0[1], p0[2]),
                Pt3::new(p1[0], p1[1], p1[2]),
                Pt3::new(p2[0], p2[1], p2[2]),
            ),
            // TODO: into reference
            mat: Rc::new(mat.clone())
        });
        i = i + 3;
    }
    let acc = FromFaceList::from_face_list(&list);
    Ok(MeshT {
        face_list: list,
        acc_structure: acc,
    })
}
