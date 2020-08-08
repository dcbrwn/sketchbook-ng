use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};

use crate::interop::events::*;
use crate::interop::log::*;

pub mod initial;

pub struct Sketchbook {}

impl Sketchbook {
    pub fn new() -> Self {
        Sketchbook {}
    }

    pub fn load_sketch(&mut self, args: String, canvas: web_sys::HtmlCanvasElement) -> () {
        log(&format!("Loading sketch '{}'...", &args));

        let sketch = match args.as_str() {
            "#initial" => Some(initial::Initial::new(canvas)),
            _ => None
        };

        if let Some(sketch) = sketch {
            attach_global_listener(Box::new(sketch));
        }
    }
}
