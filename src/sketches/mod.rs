use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};

use crate::common::log::*;
use crate::sketches::events::{
    EventTarget,
    SketchEvent,
};
use crate::sketches::initial::Initial;

mod colors;
pub mod events;
pub mod initial;

pub struct Sketchbook {
    current_sketch: Option<Box<dyn EventTarget>>,
}

impl Sketchbook {
    pub fn new() -> Self {
        Sketchbook {
            current_sketch: None,
        }
    }

    pub fn load_sketch(&mut self, args: String, canvas: web_sys::HtmlCanvasElement) -> () {
        log(&format!("Loading sketch '{}'...", &args));

        self.current_sketch = match args.as_str() {
            "#initial" => Some(Box::new(initial::Initial::new(canvas))),
            _ => None
        };
    }
}

impl EventTarget for Sketchbook {
    fn dispatch(&mut self, event: SketchEvent) -> () {
        if let Some(sketch) = &mut self.current_sketch {
            sketch.dispatch(event);
        }
    }
}
