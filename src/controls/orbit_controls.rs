extern crate cgmath;
extern crate gmath;
extern crate js_sys;
extern crate num_traits;
extern crate wasm_bindgen;
extern crate web_sys;
use gmath::{vec2, vec3};
use js_sys::Math;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent, WheelEvent};

use wasm_bindgen::prelude::Closure;

use crate::camera::Camera;
use crate::utils::log_num;

pub struct OrbitControls {
    dom_element: web_sys::HtmlCanvasElement,
    pub target: [f32; 3],
    rotate_start: Rc<RefCell<[f32; 2]>>,
    rotate_end: Rc<RefCell<[f32; 2]>>,
    rotate_delta: Rc<RefCell<[f32; 2]>>,

    phi_delta: Rc<RefCell<f32>>,
    theta_delta: Rc<RefCell<f32>>,
    scala: Rc<RefCell<f32>>,
    state: Rc<RefCell<i16>>,
}

impl OrbitControls {
    pub fn new(dom_element: web_sys::HtmlCanvasElement) -> OrbitControls {
        let control = OrbitControls {
            dom_element: dom_element,
            target: vec3::new_zero(),
            rotate_start: Rc::new(RefCell::new(vec2::new_zero())),
            rotate_end: Rc::new(RefCell::new(vec2::new_zero())),
            rotate_delta: Rc::new(RefCell::new(vec2::new_zero())),
            phi_delta: Rc::new(RefCell::new(0.0)),
            theta_delta: Rc::new(RefCell::new(0.0)),

            scala: Rc::new(RefCell::new(1.0)),

            state: Rc::new(RefCell::new(-1)),
        };

        let event_target: EventTarget = EventTarget::from(control.dom_element.clone());

        {
            let rotate_start = control.rotate_start.clone();
            let state = control.state.clone();
            let mousedown_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                let button = event.button();
                *state.borrow_mut() = button;

                vec2::set(
                    &mut *rotate_start.borrow_mut(),
                    event.client_x() as f32,
                    event.client_y() as f32,
                );
            }) as Box<dyn FnMut(MouseEvent)>);

            event_target
                .add_event_listener_with_callback(
                    "mousedown",
                    mousedown_cb.as_ref().unchecked_ref(),
                )
                .unwrap();
            mousedown_cb.forget();
        }

        {
            let rotate_start = control.rotate_start.clone();
            let rotate_end = control.rotate_end.clone();
            let rotate_delta = control.rotate_delta.clone();

            let phi_delta = control.phi_delta.clone();
            let theta_delta = control.theta_delta.clone();

            let state = control.state.clone();

            let mousemove_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                if *state.borrow() == 0 {
                    vec2::set(
                        &mut *rotate_end.borrow_mut(),
                        event.client_x() as f32,
                        event.client_y() as f32,
                    );
                    vec2::sub(
                        &mut *rotate_delta.borrow_mut(),
                        &mut *rotate_end.borrow_mut(),
                        &mut *rotate_start.borrow_mut(),
                    );

                    let r_delta = *rotate_delta.borrow();

                    let theta_delta1 =
                        *theta_delta.borrow() - (2.0 * PI * r_delta[0] / 1800.0 * 1.0);
                    *theta_delta.borrow_mut() = theta_delta1;

                    let phi_delta1 = *phi_delta.borrow() - (2.0 * PI * r_delta[1] / 1800.0 * 1.0);
                    *phi_delta.borrow_mut() = phi_delta1;

                    let v_rotate_end = *rotate_end.borrow();
                    vec2::set(
                        &mut *rotate_start.borrow_mut(),
                        v_rotate_end[0],
                        v_rotate_end[1],
                    );

                    // log_num(*theta_delta.borrow_mut() as f64);
                    // log_num(*phi_delta.borrow_mut()  as f64);
                }
            }) as Box<dyn FnMut(MouseEvent)>);

            event_target
                .add_event_listener_with_callback(
                    "mousemove",
                    mousemove_cb.as_ref().unchecked_ref(),
                )
                .unwrap();
            mousemove_cb.forget();
        }

        {
            let scala = control.scala.clone();

            let mousewheel_cb = Closure::wrap(Box::new(move |event: WheelEvent| {
                let delta_y = event.delta_y();

                // log_num(delta_y as f64);

                let mut new_scala = *scala.borrow() + 0.06;

                if delta_y < 0.0 {
                    new_scala = *scala.borrow() / 1.06;
                }

                *scala.borrow_mut() = new_scala;
            }) as Box<dyn FnMut(WheelEvent)>);

            event_target
                .add_event_listener_with_callback("wheel", mousewheel_cb.as_ref().unchecked_ref())
                .unwrap();

            mousewheel_cb.forget();
        }

        // MOUSEUP and MOUSEOUT
        {
            let state = control.state.clone();

            let mouseup_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *state.borrow_mut() = -1;
            }) as Box<dyn FnMut(MouseEvent)>);

            event_target
                .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
                .unwrap();
            event_target
                .add_event_listener_with_callback("mouseout", mouseup_cb.as_ref().unchecked_ref())
                .unwrap();
            mouseup_cb.forget();
        }

        control
    }
    pub fn update(&self, camera: &mut Camera) {
        let LOG_NUM = "xxx";

        if LOG_NUM == "position" {
            log_num(camera.position[0] as f64);
            log_num(camera.position[1] as f64);
            log_num(camera.position[2] as f64);
        }

        let mut offset = vec3::new_zero();
        vec3::sub(&mut offset, &camera.position, &camera.target);

        if LOG_NUM == "position_0_0" {
            log_num(camera.position[0] as f64);
            log_num(camera.position[1] as f64);
            log_num(camera.position[2] as f64);
        }

        if LOG_NUM == "target" {
            log_num(camera.target[0] as f64);
            log_num(camera.target[1] as f64);
            log_num(camera.target[2] as f64);
        }

        if LOG_NUM == "offset" {
            log_num(offset[0] as f64);
            log_num(offset[1] as f64);
            log_num(offset[2] as f64);
        }

        let mut theta = Math::atan2(offset[0] as f64, offset[2] as f64);
        if LOG_NUM == "theta" {
            log_num(theta);
        }

        let sqrt = Math::sqrt((offset[0] * offset[0] + offset[2] * offset[2]).into());
        let mut phi = Math::atan2(sqrt, offset[1] as f64);
        if LOG_NUM == "phi" {
            log_num(phi);
        }

        let theta_delta = self.theta_delta.clone();
        theta = theta + *theta_delta.borrow() as f64;

        let phi_delta = self.phi_delta.clone();
        phi = phi + *phi_delta.borrow() as f64;
        if LOG_NUM == "theta_1" {
            log_num(theta);
        }
        if LOG_NUM == "phi_1" {
            log_num(phi);
        }

        // theta = theta + 0.005;
        // log_num(theta as f64);
        let EPS: f32 = 0.0001;

        phi = Math::max(EPS as f64, Math::min(((PI - EPS) as f64).into(), phi));
        if LOG_NUM == "phi_2" {
            log_num(phi);
        }

        let scala = self.scala.clone();
        let length = vec3::len(&offset) as f32;
        if LOG_NUM == "scala" {
            log_num(*scala.borrow() as f64);
        }
        // log_num(length as f64);

        let radius = length * (*scala.borrow() as f32);
        // radius = 11.0;
        if LOG_NUM == "radius" {
            log_num(radius as f64);
        }

        offset[0] = radius * (Math::sin(phi) * Math::sin(theta)) as f32;
        offset[1] = radius * Math::cos(phi) as f32;
        offset[2] = radius * (Math::sin(phi) * Math::cos(theta)) as f32;

        if LOG_NUM == "offset_1" {
            log_num(offset[0] as f64);
            log_num(offset[1] as f64);
            log_num(offset[2] as f64);
        }
        if LOG_NUM == "target_1" {
            log_num(camera.target[0] as f64);
            log_num(camera.target[1] as f64);
            log_num(camera.target[2] as f64);
        }
        if LOG_NUM == "position_0_5" {
            log_num(camera.position[0] as f64);
            log_num(camera.position[1] as f64);
            log_num(camera.position[2] as f64);
        }
        camera.position[0] = camera.target[0];
        camera.position[1] = camera.target[1];
        camera.position[2] = camera.target[2];

        if LOG_NUM == "position_1" {
            log_num(camera.position[0] as f64);
            log_num(camera.position[1] as f64);
            log_num(camera.position[2] as f64);
        }
        if LOG_NUM == "offset_2" {
            log_num(offset[0] as f64);
            log_num(offset[1] as f64);
            log_num(offset[2] as f64);
        }

        vec3::add_mut(&mut camera.position, &offset);

        if LOG_NUM == "position_2" {
            log_num(camera.position[0] as f64);
            log_num(camera.position[1] as f64);
            log_num(camera.position[2] as f64);
        }

        // mat4::set_position(&mut camera.matrix, &camera.position);

        if LOG_NUM == "target_2" {
            log_num(camera.target[0] as f64);
            log_num(camera.target[1] as f64);
            log_num(camera.target[2] as f64);
        }

        /*

        // look_at(
        //     &mut camera.matrix,
        //     &camera.position,
        //     &camera.target,
        //     &[0.0, 1.0, 0.0],
        // );

        let mat = Matrix4::look_at(
            Point3::new(camera.position[0], camera.position[1], camera.position[2]),
            Point3::new(camera.target[0], camera.target[1], camera.target[2]),
            Vector3::new(camera.up[0], camera.up[1], camera.up[2]),
        );
        // log_matrix(&mat, "mat4");

        let mut new_mat: [f32; 16] = [0.0; 16];
        let mut i: usize = 0;

        let mat4: [[f32; 4]; 4] = mat.into();
        for x in mat4.iter() {
            for y in x.iter() {
                new_mat[i] = *y;
                i = i + 1;
            }
        }

        camera.matrix = new_mat;

        */
        camera.update_matrix();

        // let mut new_mat_cam = mat4::new_identity::<f32>();
        // mat4::transpose(&mut new_mat_cam, &camera.matrix);
        // camera.matrix = new_mat_cam;

        // log_num_arr(&camera.matrix, "camera.matrix");

        *theta_delta.borrow_mut() = 0.0;
        *phi_delta.borrow_mut() = 0.0;
        *scala.borrow_mut() = 1.0;

        let state = self.state.clone();
    }
}
