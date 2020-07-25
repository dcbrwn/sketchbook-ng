#![allow(unused)]
extern crate wee_alloc;

use std::borrow::BorrowMut;
use std::cell::RefCell;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use sketches::Sketchbook;

use crate::sketches::sketch::{PointerEventData, EventTarget, TickEventData, WheelEventData, WindowResizeData};
use crate::sketches::sketch::SketchEvent::{PointerDown, PointerMove, PointerUp, Tick, Wheel, WindowResize};

mod common;
mod sketches;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    pub static BOOK: RefCell<Sketchbook> = RefCell::new(Sketchbook::new());
}

#[wasm_bindgen]
pub fn on_load(args: Option<String>, canvas: web_sys::HtmlCanvasElement) -> () {
    if let Some(actual_args) = args {
        BOOK.with(|x| {
            x.borrow_mut().load_sketch(actual_args, canvas);
        })
    }
}

#[wasm_bindgen]
pub fn on_tick(time: f64) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(Tick(TickEventData { time }));
    })
}

#[wasm_bindgen]
pub fn on_resize(width: f64, height: f64) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(WindowResize(WindowResizeData { width, height }));
    })
}

#[wasm_bindgen]
pub fn on_pointer_down(button: i8, x: f64, y: f64) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(PointerDown(PointerEventData { button, x, y }));
    })
}

#[wasm_bindgen]
pub fn on_pointer_move(button: i8, x: f64, y: f64) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(PointerMove(PointerEventData { button, x, y }));
    })
}

#[wasm_bindgen]
pub fn on_pointer_up(button: i8, x: f64, y: f64) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(PointerUp(PointerEventData { button, x, y }));
    })
}

#[wasm_bindgen]
pub fn on_wheel(dx: f64, dy: f64, px: f64, py: f64) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(Wheel(WheelEventData { dx, dy, px, py }));
    })
}
