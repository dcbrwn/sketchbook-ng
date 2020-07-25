#![allow(unused)]
extern crate wee_alloc;

use std::borrow::BorrowMut;
use std::cell::RefCell;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use sketches::Sketchbook;

use crate::sketches::events::{
    PointerEventData,
    EventTarget,
    TickEventData,
    WheelEventData,
    WindowResizeData,
    SketchEvent
};
use crate::sketches::events::SketchEvent::{
    PointerDown,
    PointerMove,
    PointerUp,
    Tick,
    Wheel,
    WindowResize
};

mod common;
mod sketches;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    pub static BOOK: RefCell<Sketchbook> = RefCell::new(Sketchbook::new());
}

fn dispatch(event: SketchEvent) -> () {
    BOOK.with(|book| {
        book.borrow_mut().dispatch(event);
    })
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
    dispatch(Tick(TickEventData { time }));
}

#[wasm_bindgen]
pub fn on_resize(width: f64, height: f64) -> () {
    dispatch(WindowResize(WindowResizeData { width, height }));
}

#[wasm_bindgen]
pub fn on_pointer_down(button: i8, x: f64, y: f64) -> () {
    dispatch(PointerDown(PointerEventData { button, x, y }));
}

#[wasm_bindgen]
pub fn on_pointer_move(button: i8, x: f64, y: f64) -> () {
    dispatch(PointerMove(PointerEventData { button, x, y }));
}

#[wasm_bindgen]
pub fn on_pointer_up(button: i8, x: f64, y: f64) -> () {
    dispatch(PointerUp(PointerEventData { button, x, y }));
}

#[wasm_bindgen]
pub fn on_wheel(dx: f64, dy: f64, px: f64, py: f64) -> () {
    dispatch(Wheel(WheelEventData { dx, dy, px, py }));
}
