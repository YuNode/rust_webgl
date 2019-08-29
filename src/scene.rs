
use crate::mesh::Mesh;

pub struct Scene {
    pub objects: Vec<Mesh>,
}

impl Scene {
    pub fn add_object(&mut self, object: Mesh) {
        self.objects.push(object);
    }
}