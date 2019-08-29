extern crate cgmath;
extern crate gmath;
extern crate js_sys;
extern crate num_traits;
extern crate wasm_bindgen;
extern crate web_sys;
use gmath::mat4;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, KeyboardEvent, MouseEvent};

use wasm_bindgen::prelude::Closure;

use crate::camera::Camera;

pub struct FirstPersonControls {
    dom_element: web_sys::HtmlCanvasElement,
    drag: Rc<std::cell::RefCell<bool>>,
    key_code: Rc<std::cell::RefCell<u32>>,
}

impl FirstPersonControls {
    pub fn new(dom_element: web_sys::HtmlCanvasElement) -> FirstPersonControls {
        let control = FirstPersonControls {
            dom_element: dom_element,
            drag: Rc::new(RefCell::new(false)),
            key_code: Rc::new(RefCell::new(0)),
        };

        let event_target: EventTarget = EventTarget::from(control.dom_element.clone());

        {
            let drag = control.drag.clone();
            let mousedown_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *drag.borrow_mut() = true;
            }) as Box<dyn FnMut(MouseEvent)>);
            event_target
                .add_event_listener_with_callback(
                    "mousedown",
                    mousedown_cb.as_ref().unchecked_ref(),
                )
                .unwrap();
            mousedown_cb.forget();
        }

        // MOUSEUP and MOUSEOUT
        {
            let drag = control.drag.clone();
            let mouseup_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *drag.borrow_mut() = false;
            }) as Box<dyn FnMut(MouseEvent)>);
            event_target
                .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
                .unwrap();
            event_target
                .add_event_listener_with_callback("mouseout", mouseup_cb.as_ref().unchecked_ref())
                .unwrap();
            mouseup_cb.forget();
        }

        //KEYDOWN
        {
            let key_code = control.key_code.clone();
            let keydown_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                *key_code.borrow_mut() = event.key_code();
            }) as Box<dyn FnMut(KeyboardEvent)>);
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .add_event_listener_with_callback("keydown", keydown_cb.as_ref().unchecked_ref())
                .unwrap();
            keydown_cb.forget();
        }
        {
            let key_code = control.key_code.clone();
            let keyup_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                *key_code.borrow_mut() = 0;
            }) as Box<dyn FnMut(KeyboardEvent)>);
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .add_event_listener_with_callback("keyup", keyup_cb.as_ref().unchecked_ref())
                .unwrap();
            keyup_cb.forget();
        }
        control
    }
    pub fn update(&self, camera: &mut Camera) {
        // if *self.drag.borrow() {
        //     ////1
        //     // let newPos = [camera.position[0], camera.position[1], -3.0];
        //     // vec3::add_mut(&mut camera.position, &newPos);

        //     //2
        //     camera.position[2] = camera.position[2] - 0.1;
        // }

        // log_num(*self.key_code.borrow() as f64);

        let move_info = match *self.key_code.borrow() {
            87 => (2, 0.1),
            83 => (2, -0.1),
            65 => (0, 0.1),
            68 => (0, -0.1),
            82 => (1, -0.1),
            70 => (1, 0.1),
            _ => (0, 0.0),
        };
        if *self.key_code.borrow() != 0 {
            camera.position[move_info.0] = camera.position[move_info.0] + move_info.1;
        }

        mat4::set_position(&mut camera.matrix, &camera.position);
    }
}
