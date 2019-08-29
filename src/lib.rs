extern crate cgmath;
extern crate gmath;
extern crate js_sys;
extern crate num_traits;
extern crate wasm_bindgen;
extern crate wavefront_obj;
extern crate web_sys;
use futures::{future, Future};
use gmath::{mat4, quat, vec2, vec3};
use js_sys::{Math,Promise};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture,future_to_promise};

#[allow(dead_code)]
mod utils;
use utils::{get_string_from_xhr,request_animation_frame};

mod geometry;
use geometry::Geometry;

mod material;
use material::Material;

mod mesh;
use mesh::Mesh;

mod scene;
use scene::Scene;

mod camera;
use camera::Camera;

mod webgl_renderer;
use webgl_renderer::WebGLRenderer;

mod controls;
use controls::orbit_controls::OrbitControls;

mod loaders;


#[allow(non_snake_case)]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    //scene
    let mut mesh = get_mesh();
    // let angele = 0.71;
    // quat::rotate_x_mut(&mut mesh.rotation, &angele);
    // mesh.position[0] = 1.5;
    // mesh.position[1] = 1.5;
    // mesh.position[2] = 1.5;

    let mut mesh2 = get_mesh_2();
    // mesh2.scale=[3.0,3.0,3.0];

    let mut mesh3 = get_mesh();
    mesh.position[0] = 1.5;

    let mut mesh4 = get_mesh();
    mesh4.position[0] = -1.5;

    let mut mesh5 = get_mesh();
    mesh5.position[2] = 1.5;

    let mut scene = Scene {
        objects: vec![mesh2, mesh, mesh3, mesh4, mesh5], //,
    };

    // let mut mesh7 = get_mesh();
    // mesh7.position[0] = -1.5;
    // mesh7.position[2] = -1.5;
    // scene.add_object(mesh7);

    for x in 0..700 {
        let mut mesh = get_mesh();
        mesh.position[0] = (Math::random() * 160.0 - 80.0) as f32;
        mesh.position[1] = 0.0;
        mesh.position[2] = (Math::random() * 160.0 - 80.0) as f32;
        scene.add_object(mesh);
    }
    for x in 0..30 {
        let mut mesh = get_mesh();
        mesh.position[1] = (x as f32) * 5.0;
        mesh.material.color = [0.0, 0.0, 1.0, 1.0];
        scene.add_object(mesh);
    }

    //renderer
    let webGLRenderer = WebGLRenderer::new();

    //camera
    let mut camera = Camera::new(&webGLRenderer.dom_element);
    camera.position[0] = 24.0;
    camera.position[1] = 10.0;
    camera.position[2] = 24.0;

    camera.target[0] = 0.0;
    camera.target[1] = 5.0;
    camera.target[2] = 0.0;

    // mat4::look_at(
    //     &mut camera.matrix,
    //     &camera.position,
    //     &[0.0, 10.1, 0.0],
    //     &[0.0, 1.0, 0.0],
    // );

    // mat4::look_at(
    //     &mut camera.matrix,
    //     &[12_f32, 1_f32, 12_f32],
    //     &[0_f32, 0_f32, 0_f32],
    //     &[0_f32, 1_f32, 0_f32],
    // );
    //control
    let mut control = OrbitControls::new(webGLRenderer.dom_element.clone());

    let start_render = |meshs: Vec<Mesh>| {
        scene.objects = Vec::new();

        for mesh in meshs {
            scene.add_object(mesh);
        }

        //render 1 request_animation_frame start
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let cb = move |dt: f32| {
            // let angele = 0.01;
            // quat::rotate_z_mut(&mut scene.objects[0].rotation, &angele);

            control.update(&mut camera);

            webGLRenderer
                .render(&mut scene, &mut camera)
                .expect("render err");

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        };

        *g.borrow_mut() = Some(Closure::wrap(Box::new(cb) as Box<FnMut(f32)>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        //render 1 request_animation_frame end

        //render 2 render once

        // control.update(&mut camera);
        // webGLRenderer.render(&mut scene, &mut camera);
    };


    //model/Duck.gltf model/rust_logo2.obj model/SimpleSkinning.obj model/SimpleSkinning.obj
    //model/CesiumMilkTruck.gltf model/BoxVertexColors.gltf
    let future =JsFuture::from(get_string_from_xhr("model/WaltHead.obj"))
    .and_then(|js_str|{
            let meshs = loaders::obj::load_obj_string(&js_str.as_string().unwrap()).unwrap();
            start_render(meshs);
            JsFuture::from(Promise::resolve(&JsValue::NULL))

    }); 
    future_to_promise(future);

    Ok(())
}

fn cal(num: i64, sum: f64) -> f64 {
    let sum1 = sum * 1.1012345678;

    if num == 0 {
        return sum1 + 1.0;
    } else {
        return cal(num - 1, sum1);
    }
}

fn get_mesh_2() -> Mesh {
    let vertices: [f32; 9] = [12.0, 0.0, 0.0, 0.0, 0.0, 12.0, -12.0, 0.0, 0.0];

    let indices: [u16; 3] = [0, 1, 2];

    let normals: [f32; 9] = [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0];

    let geometry = Geometry {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
        normals: normals.to_vec(),
    };

    let material = Material {
        color: [1.0, 0.0, 1.0, 1.0],
    };

    let mesh = Mesh {
        geometry: geometry,
        material: material,
        position: vec3::new_zero(),
        scale: vec3::new_one(),
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

    mesh
}

fn get_mesh() -> Mesh {
    let vertices: [f32; 72] = [
        1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, // v0-v1-v2-v3 front
        1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, // v0-v3-v4-v5 right
        1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, // v0-v5-v6-v1 up
        -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        1.0, // v1-v6-v7-v2 left
        -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
        1.0, // v7-v4-v3-v2 down
        1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        -1.0, // v4-v7-v6-v5 back
    ];

    let indices: [u16; 36] = [
        0, 1, 2, 0, 2, 3, // front
        4, 5, 6, 4, 6, 7, // back
        8, 9, 10, 8, 10, 11, // top
        12, 13, 14, 12, 14, 15, // bottom
        16, 17, 18, 16, 18, 19, // right
        20, 21, 22, 20, 22, 23, // left
    ];

    let normals: [f32; 72] = [
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, // v0-v1-v2-v3 front
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // v0-v3-v4-v5 right
        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // v0-v5-v6-v1 up
        -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // v1-v6-v7-v2 left
        0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, // v7-v4-v3-v2 down
        0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, // v4-v7-v6-v5 back
    ];

    let geometry = Geometry {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
        normals: normals.to_vec(),
    };

    let material = Material {
        color: [1.0, 0.0, 0.0, 1.0],
    };

    let mesh = Mesh {
        geometry: geometry,
        material: material,
        position: vec3::new_zero(),
        scale: vec3::new_one(),
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

    mesh
}
