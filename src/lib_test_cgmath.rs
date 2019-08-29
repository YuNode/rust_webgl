extern crate cgmath;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
use self::cgmath::prelude::*;
use self::cgmath::{
    ortho, vec3, Decomposed, Deg, Euler, Matrix3, Matrix4, PerspectiveFov, Point3, Quaternion, Rad,
    Vector2, Vector3, Vector4,
};
use js_sys::{Float32Array, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, set_panic_hook};

fn log_num_arr(&num_arr: &[f32; 16], name: &str) {
    web_sys::console::log_1(&name.into());
    let mut i: i32 = 0;
    for x in num_arr.iter() {
        let mut item_str = String::from(x.to_string());
        item_str = i.to_string() + ":" + &item_str;
        web_sys::console::log_1(&item_str.into());
        i = i + 1;
    }
}

fn log_matrix(&mat: &Matrix4<f64>, name: &str) {
    let mat4: [[f64; 4]; 4] = mat.into();

    let mut i: i32 = 0;
    for x in mat4.iter() {
        for y in x.iter() {
            let mut item_str = String::from(y.to_string());
            item_str = i.to_string() + ":" + &item_str;
            web_sys::console::log_1(&item_str.into());
            i = i + 1;
        }
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();

    let mat = Matrix4::look_at(
        Point3::new(12.0, 0.0, 12.0),
        Point3::new(0.0, 5.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    // log_matrix(&mat, "mat4");

    let mut new_mat: [f32; 16] = [0.0; 16];
    let mut i: usize = 0;

    let mat4: [[f64; 4]; 4] = mat.into();
    for x in mat4.iter() {
        for y in x.iter() {
            new_mat[i] = *y as f32;
            i = i + 1;
        }
    }

    log_num_arr(&new_mat, "new_mat");


    /*============ Creating a canvas =================*/
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    /*==========Defining and storing the geometry=======*/

    let vertices: [f32; 9] = [
        -0.5, 0.5, 0.0, //
        0.0, 0.5, 0.0, //
        -0.25, 0.25, 0.0, //
    ];
    let vertices_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let location: u32 = vertices.as_ptr() as u32 / 4;
        Float32Array::new(&memory_buffer).subarray(location, location + vertices.len() as u32)
    };

    // Create an empty buffer object to store the vertex buffer
    let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;

    //Bind appropriate array buffer to it
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Pass the vertex data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    /*=========================Shaders========================*/

    // vertex shader source code
    let vertCode = r#"attribute vec3 coordinates;
void main(void) {
    gl_Position = vec4(coordinates, 1.0);
    gl_PointSize = 10.0;
}
"#;
    // Create a vertex shader object
    let vertShader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vertCode)?;

    // fragment shader source code
    let fragCode = r#"void main(void) {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);
}"#;
    // Create fragment shader object
    let fragShader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, fragCode)?;
    // Link both programs
    let shaderProgram = link_program(&gl, &vertShader, &fragShader)?;
    // Use the combined shader program object
    gl.use_program(Some(&shaderProgram));

    /*======== Associating shaders to buffer objects ========*/

    // Bind vertex buffer object
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Get the attribute location
    let coord = gl.get_attrib_location(&shaderProgram, "coordinates") as u32;

    // Point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer_with_i32(coord, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    // Enable the attribute
    gl.enable_vertex_attrib_array(coord);

    /*============= Drawing the primitive ===============*/

    // Clear the canvas
    gl.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    // Clear the color buffer bit
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // Set the view port
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // Draw the triangle
    gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 3);

    Ok(())
}
