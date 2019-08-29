extern crate cgmath;
extern crate gmath;
extern crate js_sys;
extern crate num_traits;
extern crate wasm_bindgen;
extern crate web_sys;
use gmath::{mat4, vec3};
use std::f32::consts::PI;


use self::cgmath::{
     Point3, Vector3,Matrix4
};

pub struct Camera {
    pub position: [f32; 3],
    pub up: [f32; 3],
    pub target: [f32; 3],
    pub matrix: [f32; 16],
    pub projection_matrix: [f32; 16],
}

#[allow(non_snake_case)]
impl Camera {
    pub fn new(dom_element: &web_sys::HtmlCanvasElement) -> Camera {
        fn get_projection(angle: f32, a: f32, zMin: f32, zMax: f32) -> [f32; 16] {
            let ang = ((angle * 0.5) * PI / 180.0).tan(); //angle*0.5
            [
                0.5 / ang,
                0.0,
                0.0,
                0.0,
                0.0,
                0.5 * a / ang,
                0.0,
                0.0,
                0.0,
                0.0,
                -(zMax + zMin) / (zMax - zMin),
                -1.0,
                0.0,
                0.0,
                (-2.0 * zMax * zMin) / (zMax - zMin),
                0.0,
            ]
        }

        //1
        let proj_matrix: [f32; 16] = get_projection(
            40.0,
            (dom_element.width() as f32 / dom_element.height() as f32) as f32,
            1.0,
            100.0,
        );
        //2
        let fieldOfView = 45.0 * PI / 180.0; // in radians
        let aspect: f32 = dom_element.width() as f32 / dom_element.height() as f32;
        let zNear = 0.01;
        let zFar = 1000.0;
        let mut projectionMatrix = mat4::new_identity();
        mat4::perspective(&mut projectionMatrix, &fieldOfView, &aspect, &zNear, &zFar);
        Camera {
            position: vec3::new_zero(),
            up: [0.0, 1.0, 0.0],
            target: vec3::new_zero(),
            matrix: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ],
            // 1.0, 0.0, 0.0, 0.0, //
            // 0.0, 1.0, 0.0, 0.0, //
            // 0.0, 0.0, 1.0, 0.0, //
            // 12.0, 0.0, 12.0, 1.0,

            // 1.0, 0.0, 0.0, 0.0, //
            // 0.0, 1.0, 0.0, 0.0, //
            // 0.0, 0.0, 1.0, -0.1, //
            // 0.0, 0.0, -2.0, 1.0, //translating z
            projection_matrix: projectionMatrix, //proj_matrix projectionMatrix
        }
    }
    pub fn update_matrix(&mut self) {
        let mat = Matrix4::look_at(
            Point3::new(self.position[0], self.position[1], self.position[2]),
            Point3::new(self.target[0], self.target[1], self.target[2]),
            Vector3::new(self.up[0], self.up[1], self.up[2]),
        );

        let mut new_mat: [f32; 16] = [0.0; 16];
        let mut i: usize = 0;

        let mat4: [[f32; 4]; 4] = mat.into();
        for x in mat4.iter() {
            for y in x.iter() {
                new_mat[i] = *y;
                i = i + 1;
            }
        }

        self.matrix = new_mat;
    }
}