use crate::common::dom::*;
use crate::common::log::*;

pub mod colors;
pub mod sketch;
pub mod initial;

pub fn get_sketch() -> Option<Box<dyn sketch::Sketch>> {
    let loc = window().location().hash().unwrap();

    log(&format!("Loading sketch '{}'...", &loc));

    return match loc.as_str() {
        "#initial" => Some(Box::new(initial::Initial::new())),
        _ => None
    };
}
