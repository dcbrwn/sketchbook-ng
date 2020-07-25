#![allow(unused)]

mod common;
mod sketches;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use common::dom::{window, request_animation_frame};
use sketches::{get_sketch};

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    let performance = window()
        .performance()
        .expect("performance should be available");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut sketch = get_sketch().unwrap();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let t = performance.now() / 1000.0;

        sketch.tick(t);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
