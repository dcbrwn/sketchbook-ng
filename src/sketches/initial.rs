use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// use crate::common::log::*;
use super::sketch::*;
use crate::common::dom::{document};

pub struct Initial {
    context: web_sys::CanvasRenderingContext2d,
}

impl Initial {
    pub fn new() -> Self {
        let canvas = document().get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Initial {
            context: context,
        }
    }
}

impl Sketch for Initial {
    fn tick(&self, t: f64) {
        let context = &self.context;

        context.set_fill_style(&JsValue::from("white"));

        context.fill_rect(0.0, 0.0, 1000.0, 1000.0);
        context.begin_path();

        let x = f64::cos(t * f64::consts::PI * 2.0) * 50.0;
        let y = f64::sin(t * f64::consts::PI * 2.0) * 50.0;

        context
            .arc(90.0 + x, 65.0 + y, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    }
}
