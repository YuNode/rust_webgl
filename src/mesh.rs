use crate::geometry::Geometry;
use crate::material::Material;
use std::sync::Arc;
extern crate gmath;
use gmath::{mat4, quat, vec2, vec3};

#[allow(non_snake_case)]
pub struct Mesh {
    pub geometry: Geometry,
    pub material: Material,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 4],
    pub matrix: [f32; 16],
    pub __webGLVertexBuffer: Option<web_sys::WebGlBuffer>,
    pub __webGLFaceBuffer: Option<web_sys::WebGlBuffer>,
    pub __webGLNormalBuffer: Option<web_sys::WebGlBuffer>,
    pub __webGLColorBuffer: Option<web_sys::WebGlBuffer>,
}

impl Mesh {
    pub fn from_gltf(
        buffers: &[gltf::buffer::Data],
        primitive: &gltf::Primitive,
        materials: &[Arc<Material>],
        transform: [f32; 16],
    ) -> Self {
        // We're only dealing with triangle meshes
        assert_eq!(
            gltf::mesh::Mode::Triangles,
            primitive.mode(),
            "Not handling non-triangle glTF primitives"
        );

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let indices = reader
            .read_indices()
            .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
            .expect("Failed to read glTF indices");
        let positions: Vec<_> = reader
            .read_positions()
            .expect("Failed to read glTF positions")
            .map(|data| data)
            .collect();
        let normals: Vec<_> = reader
            .read_normals()
            .expect("Failed to read glTF normals")
            .map(|data| data)
            .collect();
        let material = primitive.material().index().map(|id| materials[id].clone()).unwrap_or_default();
        // Not handling optional normals yet
        assert_eq!(
            positions.len(),
            normals.len(),
            "Position vector and normals vector have different lengths"
        );

        fn to1dVec<T: Copy>(arr2d: Vec<[T; 3]>) -> Vec<T> {
            let mut vec = Vec::new();
            for subVec in arr2d {
                vec.push(subVec[0]);
                vec.push(subVec[1]);
                vec.push(subVec[2]);
            }
            vec
        }

        let mut newIndeces = Vec::new();
        for value in indices {
            newIndeces.push(value as u16);
        }

        let geometry = Geometry {
            vertices: to1dVec(positions),
            indices: newIndeces,
            normals: to1dVec(normals),
        };
        let material1 = Material {
            color: [1.0, 0.0, 1.0, 1.0],
        };
        let mesh = Mesh {
            geometry: geometry,
            material: (*material).clone(),
            position: vec3::new_zero(),
            scale: vec3::new_one(),
            rotation: quat::new_identity(),
            matrix: transform,
            __webGLVertexBuffer: None,
            __webGLFaceBuffer: None,
            __webGLNormalBuffer: None,
            __webGLColorBuffer: None,
        };

        mesh
    }
}
