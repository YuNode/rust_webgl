extern crate cgmath;
extern crate gmath;
extern crate js_sys;
extern crate num_traits;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::{JsCast};
use web_sys::{Request, RequestInit, RequestMode, Response,WebGlRenderingContext};
use wasm_bindgen_futures::{JsFuture,future_to_promise};
use futures::{future, Future};
use js_sys::Promise;

use wasm_bindgen::prelude::Closure;

use web_sys::{WebGlProgram, WebGlShader};
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<FnMut(f32)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn log_num(num: f64) {
    let v = wasm_bindgen::JsValue::from_f64(num);
    web_sys::console::log_1(&v);
}

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

pub fn get_string_from_xhr(url: &str) -> Promise  {

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.text()
        })
        .and_then(|js_value_str: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(js_value_str)
        })
        .and_then(|js_value_str| {
            // web_sys::console::log_1(&js_value_str);

            //** ob parse *//
            //let meshs = loaders::obj::load_obj_string(&js_value_str.as_string().unwrap()).unwrap();
            // start_render(meshs);
            // cb(meshs);

            /*
            /**gltf parse */
            let meshs = loaders::gltf::load_gltf_string(&js_value_str.as_string().unwrap()).unwrap();
            start_render(meshs);
            */

            // let mut cl = move |resolve: js_sys::Function, reject: js_sys::Function| {};
            // let promise_cb: &mut dyn std::ops::FnMut(js_sys::Function, js_sys::Function) = &mut cl;
            // JsFuture::from(Promise::new(promise_cb))

            JsFuture::from(Promise::resolve(&js_value_str))
        });

    future_to_promise(future)
}

