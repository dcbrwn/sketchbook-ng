use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn get_canvas_by_id(id: &str) -> web_sys::HtmlCanvasElement {
    let canvas = document().get_element_by_id(id).unwrap();

    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}
