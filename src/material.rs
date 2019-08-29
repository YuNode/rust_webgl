use crate::utils::log_num;

#[derive(Clone)]
pub struct Material {
    pub color: [f32; 4],
}
impl Default for Material {
    fn default() -> Self {
        Self {
            color: [0.5, 0.5, 0.5, 1.0],
        }
    }
}

impl<'a> From<gltf::Material<'a>> for Material {
    fn from(mat: gltf::Material<'a>) -> Self {
        let [r, g, b, a] = mat.pbr_metallic_roughness().base_color_factor();
        log_num(r as f64);
        log_num(g as f64);
        log_num(b as f64);

        Self {
            color: [r, g, b, a],
        }
    }
}
