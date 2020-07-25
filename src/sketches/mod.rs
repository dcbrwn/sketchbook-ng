use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};

use crate::common::log::*;
use crate::sketches::initial::Initial;
use crate::sketches::sketch::{
    EventTarget,
    PointerEventData,
    SketchEvent,
    TickEventData,
    WheelEventData
};

mod colors;
pub mod sketch;
pub mod initial;

pub struct Sketchbook {
    current_sketch: Option<RefCell<Box<dyn EventTarget>>>,
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
            "#initial" => Some(RefCell::new(Box::new(initial::Initial::new(canvas)))),
            _ => None
        };
    }
}

impl EventTarget for Sketchbook {
    fn dispatch(&mut self, event: SketchEvent) -> () {
        if let Some(sketch) = &self.current_sketch {
            sketch.borrow_mut().dispatch(event);
        }
    }
}
