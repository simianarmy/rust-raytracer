/**
 * Wavefront OBJ file parser
 */
use crate::math::*;
use crate::object::*;
use crate::shapes::shape::*;
use crate::shapes::triangle::*;
use crate::tuple::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use tobj::*;

type GroupMap = HashMap<String, Object>;

pub struct ObjData {
    groups: GroupMap,
    raw: Vec<Model>,
}

const DEFAULT_GROUP_KEY: &str = "default";

impl ObjData {
    fn make_vertex(positions: &[f32], idx: usize) -> Point {
        point(
            positions[idx] as F3D,
            positions[idx + 1] as F3D,
            positions[idx + 2] as F3D,
        )
    }

    fn make_triangle(positions: &[f32], indices: &[u32], i: usize) -> Object {
        let mut idx: usize = 3 * indices[i] as usize;
        let p1 = ObjData::make_vertex(positions, idx);

        idx = 3 * indices[i + 1] as usize;
        let p2 = ObjData::make_vertex(positions, idx);

        idx = 3 * indices[i + 2] as usize;
        let p3 = ObjData::make_vertex(positions, idx);

        triangle(p1, p2, p3)
    }

    fn fan_triangulation(positions: &[f32], indices: &[u32]) -> Vec<Object> {
        let mut triangles = vec![];
        for idx in 1..(indices.len() - 1) {
            let p1 = ObjData::make_vertex(positions, 3 * indices[0] as usize);
            let p2 = ObjData::make_vertex(positions, 3 * indices[idx] as usize);
            let p3 = ObjData::make_vertex(positions, 3 * indices[idx + 1] as usize);
            triangles.push(triangle(p1, p2, p3));
        }
        triangles
    }

    pub fn new(models: Vec<Model>) -> Self {
        // Generate group children
        let mut groups = GroupMap::new();

        for (_, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut triangles = vec![];

            if mesh.face_arities.len() > 0 {
                let mut next_face = 0;
                for face in 0..mesh.face_arities.len() {
                    let end = next_face + mesh.face_arities[face] as usize;
                    let face_indices = &mesh.indices[next_face..end];
                    // apply fan triangulation to the polygon vertices indexed by face_indices
                    triangles.extend(ObjData::fan_triangulation(&mesh.positions, &face_indices));
                    next_face = end;
                }
            } else {
                for j in 0..(mesh.indices.len() / 3) {
                    let idx = j * 3;
                    triangles.push(ObjData::make_triangle(&mesh.positions, &mesh.indices, idx));
                }
            }
            let hash_key = if m.name != "unnamed_object" {
                m.name.as_str()
            } else {
                DEFAULT_GROUP_KEY
            };
            groups.insert(hash_key.to_string(), Object::new_group(triangles));
        }

        Self {
            groups,
            raw: models,
        }
    }

    pub fn default_group(&self) -> Option<&Object> {
        self.groups.get(&DEFAULT_GROUP_KEY.to_string())
    }

    pub fn to_group(&self) -> Object {
        let mut gs = vec![];

        for (_, g) in &self.groups {
            gs.push(g.clone());
        }
        if gs.len() > 1 {
            Object::new_group(gs)
        } else if gs.len() == 1 {
            // just return the lone group object
            gs[0].clone()
        } else {
            panic!("no groups!");
        }
    }
}

const LOAD_OPTIONS: LoadOptions = OFFLINE_RENDERING_LOAD_OPTIONS; // tobj::GPU_LOAD_OPTIONS; // &tobj::LoadOptions::default()

pub fn parse_obj_file(filename: &str) -> Result<ObjData, Error> {
    let (models, _) = tobj::load_obj(&filename, &LOAD_OPTIONS).expect("Failed to OBJ load file");

    println!("models: {:?}", models);
    Ok(ObjData::new(models))
}

fn debug_model(models: &Vec<Model>) {
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("");
        println!("model[{}].name             = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!(
            "model[{}].face_count       = {}",
            i,
            mesh.face_arities.len()
        );
        let mut next_face = 0;
        for face in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[face] as usize;

            let face_indices = &mesh.indices[next_face..end];
            println!(" face[{}].indices          = {:?}", face, face_indices);

            if !mesh.texcoord_indices.is_empty() {
                let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                println!(
                    " face[{}].texcoord_indices = {:?}",
                    face, texcoord_face_indices
                );
            }
            if !mesh.normal_indices.is_empty() {
                let normal_face_indices = &mesh.normal_indices[next_face..end];
                println!(
                    " face[{}].normal_indices   = {:?}",
                    face, normal_face_indices
                );
            }

            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in
        // this example.
        println!(
            "model[{}].positions        = {}",
            i,
            mesh.positions.len() / 3
        );
        assert!(mesh.positions.len() % 3 == 0);

        /*
        for vtx in 0..mesh.positions.len() / 3 {
            println!(
                "              position[{}] = ({}, {}, {})",
                vtx,
                mesh.positions[3 * vtx],
                mesh.positions[3 * vtx + 1],
                mesh.positions[3 * vtx + 2]
            );
        }
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "obj_file";

    fn test_filename(id: &str) -> String {
        format!("tests/{}-{}.obj", TEST_FILE, id)
    }

    fn write_obj_file(filename: &str, contents: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    #[test]
    fn ignoring_unrecognized_lines() {
        let fname = test_filename("ignores");
        write_obj_file(
            fname.as_str(),
            "\
        blah blah blah\n\
        blah\n\
        blabby\n\
        blabla\n\
        ",
        )
        .unwrap();
        match parse_obj_file(fname.as_str()) {
            Ok(data) => assert_eq!(data.raw.len(), 1),
            _ => (),
        }
    }

    #[test]
    fn parsing_triangle_faces() {
        let filedata = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4
";
        let fname = test_filename("triangle-faces");
        write_obj_file(fname.as_str(), filedata).unwrap();

        match parse_obj_file(fname.as_str()) {
            Ok(data) => {
                debug_model(&data.raw);
                match data.default_group().unwrap().shape() {
                    Shape::Group(g) => {
                        assert_eq!(g.children().len(), 2);
                        let t1 = g.children()[0].clone();
                        let t2 = g.children()[1].clone();
                        match t1.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(-1.0, 0.0, 0.0));
                                assert_eq!(t.p3, point(1.0, 0.0, 0.0));
                            }
                            _ => panic!(),
                        }
                        match t2.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(1.0, 0.0, 0.0));
                                assert_eq!(t.p3, point(1.0, 1.0, 0.0));
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            Err(e) => {
                println!("parse error {:?}", e);
                panic!("load error");
            }
        }
    }

    #[test]
    fn triangulating_polygons() {
        let filedata = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5
";
        let fname = test_filename("triangulate-poly");
        write_obj_file(fname.as_str(), filedata).unwrap();

        match parse_obj_file(fname.as_str()) {
            Ok(data) => {
                debug_model(&data.raw);
                match data.default_group().unwrap().shape() {
                    Shape::Group(g) => {
                        assert_eq!(g.children().len(), 3);
                        let t1 = g.children()[0].clone();
                        let t2 = g.children()[1].clone();
                        let t3 = g.children()[2].clone();
                        match t1.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(-1.0, 0.0, 0.0));
                                assert_eq!(t.p3, point(1.0, 0.0, 0.0));
                            }
                            _ => panic!(),
                        }
                        match t2.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(1.0, 0.0, 0.0));
                                assert_eq!(t.p3, point(1.0, 1.0, 0.0));
                            }
                            _ => panic!(),
                        }
                        match t3.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(1.0, 1.0, 0.0));
                                assert_eq!(t.p3, point(0.0, 2.0, 0.0));
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            Err(e) => {
                println!("parse error {:?}", e);
                panic!("load error");
            }
        }
    }

    #[test]
    fn triangles_in_groups() {
        let filedata = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";
        let fname = test_filename("groups");
        write_obj_file(fname.as_str(), filedata).unwrap();

        match parse_obj_file(fname.as_str()) {
            Ok(data) => {
                let g1 = data.groups.get("FirstGroup").unwrap();
                match g1.shape() {
                    Shape::Group(g) => {
                        assert_eq!(g.children().len(), 1);
                        let t1 = g.children()[0].clone();
                        match t1.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(-1.0, 0.0, 0.0));
                                assert_eq!(t.p3, point(1.0, 0.0, 0.0));
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
                let g2 = data.groups.get("SecondGroup").unwrap();
                match g2.shape() {
                    Shape::Group(g) => {
                        assert_eq!(g.children().len(), 1);
                        let t2 = g.children()[0].clone();
                        match t2.shape() {
                            Shape::Triangle(t) => {
                                assert_eq!(t.p1, point(-1.0, 1.0, 0.0));
                                assert_eq!(t.p2, point(1.0, 0.0, 0.0));
                                assert_eq!(t.p3, point(1.0, 1.0, 0.0));
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
