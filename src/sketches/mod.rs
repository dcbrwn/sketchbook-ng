use crate::common::dom::*;
use crate::common::log::*;

mod sketch;
mod initial;
mod plotter_example;

pub fn get_sketch() -> Option<Box<dyn sketch::Sketch>> {
    let loc = window().location().hash().unwrap();

    log(&format!("Loading sketch '{}'...", &loc));

    return match loc.as_str() {
        "#initial" => Some(Box::new(initial::Initial {})),
        "#plotter_example" => Some(Box::new(plotter_example::PlotterExample {})),
        _ => None
    };
}
