extern crate cgmath;
extern crate gmath;
extern crate js_sys;
extern crate num_traits;
extern crate wasm_bindgen;
extern crate web_sys;
use crate::camera::Camera;
use crate::scene::Scene;
use gmath::mat4;
use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlUniformLocation};

use crate::utils::{compile_shader, link_program, set_panic_hook,log_num};

#[macro_export]
macro_rules! float_32_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let arr_location = $arr.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(arr_location, arr_location + $arr.len() as u32);
        array
    }};
}
#[macro_export]
macro_rules! uint_16_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let arr_location = $arr.as_ptr() as u32 / 2;
        let array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(arr_location, arr_location + $arr.len() as u32);
        array
    }};
}

pub struct WebGLRenderer {
    pub dom_element: web_sys::HtmlCanvasElement,
    _gl: web_sys::WebGlRenderingContext,
    _program: web_sys::WebGlProgram,
    pub auto_clear: bool,
}

#[allow(non_snake_case)]
impl WebGLRenderer {
    pub fn new() -> WebGLRenderer {
        fn initGL(canvas: &web_sys::HtmlCanvasElement) -> web_sys::WebGlRenderingContext {
            let gl = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<WebGlRenderingContext>()
                .unwrap();

            // Clear the canvas
            gl.clear_color(1.0, 1.0, 1.0, 1.0);
            gl.clear_depth(1.0);

            // Enable the depth test
            gl.enable(WebGlRenderingContext::DEPTH_TEST);
            gl.depth_func(WebGlRenderingContext::LEQUAL);

            gl.enable(WebGlRenderingContext::BLEND);
            gl.blend_func(
                WebGlRenderingContext::SRC_ALPHA,
                WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            );
            //gl.blendFunc( WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE ); // cool!

            // gl.clear_color(0.0, 0.0, 0.0, 0.0);

            gl
        }

        fn initProgram(_gl: &web_sys::WebGlRenderingContext) -> web_sys::WebGlProgram {
            /*=========================Shaders========================*/

            // vertex shader source code
            let vertCode = r#"
               attribute vec3 position;
               attribute vec4 color;  //表面基底色
               attribute vec3 normal;
               uniform mat4 Pmatrix;
               uniform mat4 Vmatrix;
               uniform mat4 Mmatrix;
               
               varying vec3 vColor;
               void main(void) {
                 vec3 lightColor = vec3(1.0, 1.0, 1.0);
                 vec3 lightDirection = vec3(0.5, 3.0, 4.0);  //归一化的世界坐标（入射光方向）
                 vec3 ambientLight = vec3(0.2, 0.2, 0.2);

                 
                 gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);

                 vec3 normal1 = normalize(vec3(normal));              //对法向量进行归一化
                 float nDotL = max(dot(lightDirection, normal1), 0.0);//计算光线方向和法向量的点积
                 vec3 diffuse = lightColor * vec3(color) * nDotL;     //计算漫反射光的颜色
                 vec3 ambient = ambientLight * vec3(color);
                 vColor = diffuse + ambient;  //vColor = normal;
               }
            "#;

            // Create a vertex shader object
            let vertShader =
                compile_shader(_gl, WebGlRenderingContext::VERTEX_SHADER, vertCode).unwrap();

            // fragment shader source code
            let fragCode = r#"
               precision mediump float;
               varying vec3 vColor;
               void main(void) {
                 gl_FragColor = vec4(vColor, 1.);
            }"#;
            // Create fragment shader object
            let fragShader =
                compile_shader(_gl, WebGlRenderingContext::FRAGMENT_SHADER, fragCode).unwrap();
            // Link both programs
            let _program = link_program(_gl, &vertShader, &fragShader).unwrap();
            // Use the combined shader program object
            _gl.use_program(Some(&_program));
            _program
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let _canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let gl = initGL(&_canvas);
        let _program = initProgram(&gl);

        WebGLRenderer {
            dom_element: _canvas,
            _gl: gl,
            _program: _program,
            auto_clear: true,
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.dom_element.set_width(width);
        self.dom_element.set_height(height);

        self._gl.viewport(
            0,
            0,
            self.dom_element.width() as i32,
            self.dom_element.height() as i32,
        );
    }
    pub fn clear(&self) {
        // web_sys::console::log_1(&"clear".into());

        self._gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
    }
    pub fn render(&self, scene: &mut Scene, camera: &mut Camera) -> Result<(), JsValue> {
        set_panic_hook();

        if self.auto_clear == true {
            self.clear();
        }

        for object in &mut scene.objects {
            if object.__webGLVertexBuffer.is_none() {
                /*==========Defining and storing the geometry=======*/

                // Create an empty buffer object to store the vertex buffer
                object.__webGLVertexBuffer = Some(
                    self._gl
                        .create_buffer()
                        .ok_or("failed to create buffer")
                        .unwrap(),
                );

                //Bind appropriate array buffer to it
                self._gl.bind_buffer(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    object.__webGLVertexBuffer.as_ref(),
                );

                let vertices = float_32_array!(&object.geometry.vertices);
                // web_sys::console::log_1(&vertices);

                // Pass the vertex data to the buffer
                self._gl.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &vertices,
                    WebGlRenderingContext::STATIC_DRAW,
                );

                //normal start
                object.__webGLNormalBuffer = Some(
                    self._gl
                        .create_buffer()
                        .ok_or("failed to create buffer")
                        .unwrap(),
                );

                self._gl.bind_buffer(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    object.__webGLNormalBuffer.as_ref(),
                );

                let normals = float_32_array!(&object.geometry.normals);

                self._gl.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &normals,
                    WebGlRenderingContext::STATIC_DRAW,
                );

                //normal end

                // Create an empty buffer object to store the vertex buffer
                object.__webGLColorBuffer = Some(
                    self._gl
                        .create_buffer()
                        .ok_or("failed to create buffer")
                        .unwrap(),
                );
                //Bind appropriate array buffer to it
                self._gl.bind_buffer(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    object.__webGLColorBuffer.as_ref(),
                );

                let len = &object.geometry.indices.len();

                let mut colors: Vec<f32> = Vec::new();

                for x in 0..*len {
                    colors.push(object.material.color[0]);
                    colors.push(object.material.color[1]);
                    colors.push(object.material.color[2]);
                    colors.push(object.material.color[3]);
                }
                let colors = float_32_array!(&colors);
                // web_sys::console::log_1(&colors);

                // Pass the vertex data to the buffer
                self._gl.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &colors,
                    WebGlRenderingContext::STATIC_DRAW,
                );

                // Create an empty buffer object to store Index buffer
                object.__webGLFaceBuffer = Some(
                    self._gl
                        .create_buffer()
                        .ok_or("failed to create buffer")
                        .unwrap(),
                );
                // Bind appropriate array buffer to it
                self._gl.bind_buffer(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    object.__webGLFaceBuffer.as_ref(),
                );

                let indices = uint_16_array!(&object.geometry.indices);
                // web_sys::console::log_1(&indices);

                // Pass the vertex data to the buffer
                self._gl.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    &indices,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }

            /*======== Associating shaders to buffer objects ========*/
            //顶点 start
            // Bind vertex buffer object
            self._gl.bind_buffer(
                WebGlRenderingContext::ARRAY_BUFFER,
                object.__webGLVertexBuffer.as_ref(),
            );

            // Get the attribute location
            let position = self._gl.get_attrib_location(&self._program, "position") as u32;

            // Point an attribute to the currently bound VBO
            self._gl.vertex_attrib_pointer_with_i32(
                position,
                3,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );

            // Enable the attribute
            self._gl.enable_vertex_attrib_array(position);
            //顶点 end

            //法向量 start
            self._gl.bind_buffer(
                WebGlRenderingContext::ARRAY_BUFFER,
                object.__webGLNormalBuffer.as_ref(),
            );

            let normal = self._gl.get_attrib_location(&self._program, "normal") as u32;

            self._gl.vertex_attrib_pointer_with_i32(
                normal,
                3,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );

            self._gl.enable_vertex_attrib_array(normal);
            //法向量 end

            //颜色 start
            // bind the color buffer
            self._gl.bind_buffer(
                WebGlRenderingContext::ARRAY_BUFFER,
                object.__webGLColorBuffer.as_ref(),
            );

            // get the attribute location
            let color = self._gl.get_attrib_location(&self._program, "color") as u32;

            // point attribute to the volor buffer object
            self._gl.vertex_attrib_pointer_with_i32(
                color,
                4,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );

            // enable the color attribute
            self._gl.enable_vertex_attrib_array(color);
            //颜色 end

            /*========================= MATRIX ========================= */
            mat4::compose(
                &mut object.matrix,
                &object.position,
                &object.scale,
                &object.rotation,
            );

            let mov_matrix: [f32; 16] = object.matrix;

            let view_matrix: [f32; 16] = camera.matrix;

            let proj_matrix: [f32; 16] = camera.projection_matrix;

            // let proj_matrix_js = float_32_array!(&proj_matrix);
            // web_sys::console::log_1(&proj_matrix_js);

            // let view_matrix_js = float_32_array!(&view_matrix);
            // web_sys::console::log_1(&view_matrix_js);

            let Pmatrix: WebGlUniformLocation = self
                ._gl
                .get_uniform_location(&self._program, "Pmatrix")
                .ok_or_else(|| String::from("cannot get Pmatrix"))
                .unwrap();
            let Vmatrix = self
                ._gl
                .get_uniform_location(&self._program, "Vmatrix")
                .ok_or_else(|| String::from("cannot get Vmatrix"))
                .unwrap();
            let Mmatrix = self
                ._gl
                .get_uniform_location(&self._program, "Mmatrix")
                .ok_or_else(|| String::from("cannot get Mmatrix"))
                .unwrap();

            /*============= Drawing the primitive ===============*/

            self._gl
                .uniform_matrix4fv_with_f32_array(Some(&Pmatrix), false, &proj_matrix);
            self._gl
                .uniform_matrix4fv_with_f32_array(Some(&Vmatrix), false, &view_matrix);
            self._gl
                .uniform_matrix4fv_with_f32_array(Some(&Mmatrix), false, &mov_matrix);
            // Draw the triangle
            let count = object.geometry.indices.len();
            // web_sys::console::log_1(&"count:".into());
            // log_num(count as f64);

            self._gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                count as i32,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }

        Ok(())
    }
}
