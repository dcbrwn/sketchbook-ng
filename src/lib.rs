#![allow(unused)]

mod common;
mod sketches;

use std::cell::RefCell;
use std::borrow::{BorrowMut, Borrow};
use wasm_bindgen::prelude::*;
use sketches::{get_sketch};
use wasm_bindgen::JsCast;

extern crate wee_alloc;

use crate::sketches::sketch::Sketch;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    pub static SKETCH: RefCell<Box<dyn Sketch>> = RefCell::new(get_sketch().unwrap());
}

#[wasm_bindgen]
pub fn tick(time: f64) {
    SKETCH.with(|x| {
        x.borrow_mut().tick(time);
    })
}
