use crate::material::Material;
use crate::mesh::Mesh;
extern crate wavefront_obj;
use crate::loaders::obj_to_mesh::Mesh as ObjMesh;
use gmath::{mat4, quat, vec2, vec3};
use wavefront_obj::{mtl, obj};
extern crate web_sys;

pub fn load_obj_string(string: &str) -> Result<Vec<Mesh>, ()> {
    web_sys::console::log_1(&"* load_obj_string".into());
    ObjFile::load_obj_string(string).map(|file| file.model())
}

struct ObjFile {
    objSet: obj::ObjSet,
}

impl ObjFile {
    fn load_obj_string(string: &str) -> Result<Self, ()> {
        web_sys::console::log_1(&"* obj::parse start".into());

        let objSet = obj::parse(string).unwrap();

        // let obj_len = objSet.objects.len() as f64;
        // web_sys::console::log_1(&JsValue::from_f64(obj_len));
        // // log_num(objSet.objects[0].geometry[0].shapes.len() as f64);

        // fn vertexVecToVec(vertexVec: Vec<obj::Vertex>) -> Vec<f32> {
        //     let mut vec = Vec::new();
        //     for v in vertexVec {
        //         vec.push(v.x as f32);
        //         vec.push(v.y as f32);
        //         vec.push(v.z as f32);
        //     }
        //     vec
        // }
        web_sys::console::log_1(&"* obj::parse finished".into());

        Ok(Self { objSet })
    }
    fn model(&self) -> Vec<Mesh> {
        web_sys::console::log_1(&"* ObjFile model".into());

        let mtl_msg = String::from("not have mtl");

        let mat_lib = &self.objSet
            .material_library
            .as_ref().unwrap_or(&mtl_msg);

        web_sys::console::log_1(&mat_lib.as_str().into());

        let mut meshes = Vec::new();

        for object in &self.objSet.objects {
            let obj_mesh = ObjMesh::from_object(&object, false);
            let geometry2 = obj_mesh.to_geometry();

            let material = Material {
                color: [1.0, 0.0, 1.0, 1.0],
            };

            let mesh = Mesh {
                geometry: geometry2,
                material: material,
                position: vec3::new_zero(),
                scale: [3.0, 3.0, 3.0],
                rotation: quat::new_identity(),
                matrix: [
                    1.0, 0.0, 0.0, 0.0, //
                    0.0, 1.0, 0.0, 0.0, //
                    0.0, 0.0, 1.0, 0.0, //
                    0.0, 0.0, 0.0, 1.0, //
                ],
                __webGLVertexBuffer: None,
                __webGLFaceBuffer: None,
                __webGLNormalBuffer: None,
                __webGLColorBuffer: None,
            };

            meshes.push(mesh);
        }
        meshes
    }
}
