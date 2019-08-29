use crate::material::Material;
use crate::mesh::Mesh;
use base64;
use gltf::{buffer, Document, Gltf};
use std::path::Path;
use std::sync::Arc;
use web_sys::console;

pub fn load_file(path: impl AsRef<Path>) -> Result<Vec<Mesh>, gltf::Error> {
    GltfFile::load_file(path).map(|file| file.model())
}

pub fn load_gltf_string(string: &str) -> Result<Vec<Mesh>, gltf::Error> {
    web_sys::console::log_1(&"* load_gltf_string".into());

    GltfFile::load_gltf(string).map(|file| file.model())
}

#[derive(Clone)] //Debug,
pub struct GltfFile {
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    materials: Vec<Arc<Material>>,
}

impl GltfFile {
    pub fn load_file(path: impl AsRef<Path>) -> Result<Self, gltf::Error> {
        let (document, buffers, _) = gltf::import(path)?;

        // Load all the materials first, this assumes that the material index
        // that primitive refers to is loaded in the same order as document.materials()
        let materials: Vec<_> = document
            .materials()
            .map(|material| Arc::new(Material::from(material)))
            .collect();

        Ok(Self {
            document,
            buffers,
            materials,
        })
    }
    pub fn load_gltf(gltf_str: &str) -> Result<Self, gltf::Error> {
        let gltf_data = Gltf::from_slice(gltf_str.as_bytes()).unwrap();
        let document = gltf_data.document;
        let blob = gltf_data.blob;
        // let buffers = gltf_data.buffers();

        fn import_buffer_data(
            document: &Document,
            mut blob: Option<Vec<u8>>,
        ) -> Result<Vec<buffer::Data>, gltf::Error> {
            let mut buffers = Vec::new();
            for buffer in document.buffers() {
                let mut data = match buffer.source() {
                    buffer::Source::Bin => blob.take().ok_or(gltf::Error::MissingBlob),
                    buffer::Source::Uri(uri) => {
                        let m = &uri["data:".len()..].split(";base64,").nth(1).unwrap();
                        base64::decode(&m).map_err(gltf::Error::Base64)
                    }
                }?;
                if data.len() < buffer.length() {
                    return Err(gltf::Error::BufferLength {
                        buffer: buffer.index(),
                        expected: buffer.length(),
                        actual: data.len(),
                    });
                }
                while data.len() % 4 != 0 {
                    data.push(0);
                }
                buffers.push(buffer::Data(data));
            }
            Ok(buffers)
        }

        let buffers_data = match import_buffer_data(&document, blob) {
            Err(err) => match err {
                gltf::Error::MissingBlob => {
                    console::error_1(&"Missing blob".into());
                    panic!();
                }
                gltf::Error::BufferLength {
                    buffer: _,
                    expected: _,
                    actual: _,
                } => {
                    console::error_1(&"Wrong buffer length".into());
                    panic!();
                }
                _ => {
                    console::error_1(&"Unexpected error".into());
                    panic!();
                }
            },
            Ok(buffers) => buffers,
        };

        // Load all the materials first, this assumes that the material index
        // that primitive refers to is loaded in the same order as document.materials()
        let materials: Vec<_> = document
            .materials()
            .map(|material| Arc::new(Material::from(material)))
            .collect();
            
        Ok(Self {
            document,
            buffers: buffers_data,
            materials,
        })
    }
    pub fn model(&self) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for mesh in self.document.meshes() {
            for primitive in mesh.primitives() {
                meshes.push(Mesh::from_gltf(
                    &self.buffers,
                    &primitive,
                    &self.materials,
                    // Mat4::<f32>::identity(),
                    [
                        1.0, 0.0, 0.0, 0.0, //
                        0.0, 1.0, 0.0, 0.0, //
                        0.0, 0.0, 1.0, 0.0, //
                        0.0, 0.0, 0.0, 1.0, //
                    ],
                ));
            }
        }

        meshes
    }
}
