mod common;
mod sketches;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use common::dom::{request_animation_frame};
use sketches::{get_sketch};

#[wasm_bindgen(start)]
pub fn main() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let sketch = get_sketch().unwrap();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        sketch.tick(0.0);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
